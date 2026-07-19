use topcoat::{
    Result,
    router::{layout, Slot},
    view::view,
};

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>"guess.com - Play Chess Online"</title>
                <script src="https://unpkg.com/htmx.org@1.9.12"></script>
                <script src="https://cdn.tailwindcss.com"></script>
                <style>
                    "
                    body {
                        background-color: #262522;
                        color: #ffffff;
                    }
                    .board-grid {
                        grid-template-columns: repeat(8, minmax(0, 1fr));
                        grid-template-rows: repeat(8, minmax(0, 1fr));
                    }
                    .selected-sq::after {
                        content: '';
                        position: absolute;
                        top: 0; left: 0; right: 0; bottom: 0;
                        background-color: rgba(247, 247, 105, 0.55);
                        box-shadow: inset 0 0 0 3px rgba(247, 247, 105, 0.95);
                        pointer-events: none;
                        z-index: 5;
                    }
                    .last-move-sq::after {
                        content: '';
                        position: absolute;
                        top: 0; left: 0; right: 0; bottom: 0;
                        background-color: rgba(247, 247, 105, 0.35);
                        pointer-events: none;
                        z-index: 5;
                    }
                    .chess-square {
                        transition: filter 0.15s ease-in-out;
                    }
                    .chess-square:hover {
                        filter: brightness(0.92);
                    }
                    .selected-pocket-piece {
                        box-shadow: 0 0 0 3px #f7f785 !important;
                        border-color: #f7f785 !important;
                    }
                    .drag-over {
                        box-shadow: inset 0 0 0 5px #81b64c !important;
                        filter: brightness(1.1) !important;
                    }
                    "
                </style>
            </head>
            <body class="bg-[#161512] h-screen h-[100dvh] w-screen overflow-hidden text-gray-200 font-sans flex flex-col md:flex-row antialiased select-none">
                <header class="w-full bg-[#1b1a18] border-b border-[#31312f] px-3 py-2 flex items-center justify-between md:hidden shrink-0 z-30 shadow-md">
                    <a href="/" class="flex items-center gap-2 no-underline">
                        <span class="text-xl font-black text-[#769656]">"guess"</span>
                        <span class="text-xl font-black text-white">".com"</span>
                    </a>
                    <div class="flex items-center gap-2">
                        <button
                            id="mobile-details-btn"
                            type="button"
                            class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg bg-[#262522] text-xs font-extrabold text-gray-300 hover:text-white border border-[#31312f] transition"
                        >
                            <svg class="w-4 h-4 text-[#81b64c]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                            </svg>
                            <span>"Moves & Details"</span>
                        </button>
                        <button
                            id="mobile-menu-btn"
                            type="button"
                            class="p-1.5 rounded-lg bg-[#262522] text-gray-300 hover:text-white hover:bg-[#363532] focus:outline-none border border-[#31312f] transition"
                            aria-label="Toggle menu"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    </div>
                </header>

                <div id="mobile-menu-overlay" class="fixed inset-0 z-40 bg-black/80 backdrop-blur-sm hidden md:hidden transition-opacity"></div>

                <div id="mobile-menu-drawer" class="fixed inset-y-0 left-0 z-50 w-72 bg-[#1b1a18] border-r border-[#31312f] p-6 flex flex-col justify-between transform -translate-x-full transition-transform duration-300 ease-in-out md:hidden shadow-2xl">
                    <div>
                        <div class="flex items-center justify-between pb-6 border-b border-[#31312f]">
                            <a href="/" class="flex items-center gap-2 no-underline">
                                <span class="text-2xl font-black text-[#769656]">"guess"</span>
                                <span class="text-2xl font-black text-white">".com"</span>
                            </a>
                            <button id="mobile-menu-close" type="button" class="p-1 rounded-lg text-gray-400 hover:text-white hover:bg-[#262522]">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>
                        <nav class="mt-6 space-y-2">
                            <a href="/" class="flex items-center gap-3 px-4 py-3 text-base font-bold rounded-lg text-white bg-[#262522] hover:bg-[#363532] transition border border-[#31312f]">
                                <svg class="w-5 h-5 text-[#81b64c]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 00-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 00-1 1m-6 0h6"></path>
                                </svg>
                                <span>"Play (Lobby)"</span>
                            </a>
                        </nav>
                    </div>
                    <div class="pt-6 border-t border-[#31312f] text-xs text-gray-400">
                        "Powered by Topcoat & Rust"
                    </div>
                </div>

                <div class="hidden md:flex w-64 bg-[#1b1a18] border-r border-[#31312f] flex-col justify-between shrink-0 h-full">
                    <div class="p-6">
                        <a href="/" class="flex items-center gap-2 no-underline">
                            <span class="text-3xl font-black text-[#769656]">"guess"</span>
                            <span class="text-3xl font-black text-white">".com"</span>
                        </a>
                        <nav class="mt-8 space-y-2">
                            <a href="/" class="flex items-center gap-3 px-4 py-3 text-base font-bold rounded-lg text-white bg-[#262522] hover:bg-[#363532] transition border border-[#31312f]">
                                <svg class="w-5 h-5 text-[#81b64c]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 00-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 00-1 1m-6 0h6"></path>
                                </svg>
                                <span>"Play (Lobby)"</span>
                            </a>
                        </nav>
                    </div>
                    <div class="p-6 border-t border-[#31312f] text-xs text-gray-500">
                        "Powered by Topcoat & Rust"
                    </div>
                </div>

                <div class="flex-1 flex flex-col h-full overflow-hidden min-h-0">
                    (slot.await?)
                </div>
            </body>
        </html>
    }
}

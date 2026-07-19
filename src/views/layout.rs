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
            <body class="bg-[#262522] min-h-screen text-gray-200 font-sans flex flex-col md:flex-row">
                <div class="w-full md:w-64 bg-[#1e1e1c] border-b md:border-b-0 md:border-r border-[#31312f] flex flex-col justify-between shrink-0">
                    <div class="p-6">
                        <a href="/" class="flex items-center gap-3 no-underline">
                            <span class="text-3xl font-extrabold text-[#769656]">"guess"</span>
                            <span class="text-3xl font-extrabold text-white">".com"</span>
                        </a>
                        <nav class="mt-8 space-y-2">
                            <a href="/" class="flex items-center gap-3 px-4 py-3 text-lg font-bold rounded-lg text-white hover:bg-[#2b2b29] transition">
                                <span>"Play (Lobby)"</span>
                            </a>
                        </nav>
                    </div>
                    <div class="p-6 border-t border-[#31312f] text-sm text-gray-500">
                        "Powered by Topcoat & Rust"
                    </div>
                </div>

                <div class="flex-1 flex flex-col overflow-y-auto">
                    (slot.await?)
                </div>
            </body>
        </html>
    }
}

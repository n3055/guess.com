(function() {
    let selectedSquare = null;
    let selectedSquareEl = null;
    // Setup Mobile Hamburger Menu & Details Drawer Toggles
    function setupMobileMenu() {
        const menuBtn = document.getElementById('mobile-menu-btn');
        const menuDrawer = document.getElementById('mobile-menu-drawer');
        const overlay = document.getElementById('mobile-menu-overlay');
        const menuCloseBtn = document.getElementById('mobile-menu-close');

        const detailsBtn = document.getElementById('mobile-details-btn');
        const detailsDrawer = document.getElementById('mobile-details-drawer');
        const detailsCloseBtn = document.getElementById('mobile-details-close');

        function openMenu() {
            closeDetails();
            if (menuDrawer) menuDrawer.classList.remove('-translate-x-full');
            if (overlay) overlay.classList.remove('hidden');
        }

        function closeMenu() {
            if (menuDrawer) menuDrawer.classList.add('-translate-x-full');
            if (!detailsDrawer || detailsDrawer.classList.contains('translate-x-full')) {
                if (overlay) overlay.classList.add('hidden');
            }
        }

        function openDetails() {
            closeMenu();
            if (detailsDrawer) detailsDrawer.classList.remove('translate-x-full');
            if (overlay) overlay.classList.remove('hidden');
        }

        function closeDetails() {
            if (detailsDrawer) detailsDrawer.classList.add('translate-x-full');
            if (!menuDrawer || menuDrawer.classList.contains('-translate-x-full')) {
                if (overlay) overlay.classList.add('hidden');
            }
        }

        if (menuBtn) menuBtn.onclick = openMenu;
        if (menuCloseBtn) menuCloseBtn.onclick = closeMenu;

        if (detailsBtn) detailsBtn.onclick = openDetails;
        if (detailsCloseBtn) detailsCloseBtn.onclick = closeDetails;

        if (overlay) overlay.onclick = function() {
            closeMenu();
            closeDetails();
        };
    }
    setupMobileMenu();
    document.addEventListener('htmx:afterSwap', setupMobileMenu);

    let lastMoveTime = 0;

    function sendMoveRequest(gameId, roleParam, fromSq, toSq, promo) {
        lastMoveTime = Date.now();
        htmx.ajax('POST', '/game/' + gameId + '/move?role=' + roleParam, {
            target: document.getElementById('game-container'),
            swap: 'innerHTML',
            values: {
                from_sq: fromSq,
                to_sq: toSq,
                promo: promo
            }
        });
    }

    // Establish WebSocket connection
    let ws;
    function connectWS() {
        const wsProto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        let wsHost;
        if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
            wsHost = window.location.hostname + ':3001';
        } else {
            wsHost = window.location.host;
        }
        const boardEl = document.getElementById('chess-board');
        if (!boardEl) return;
        const gameId = boardEl.getAttribute('data-game-id');

        console.log('Connecting to WebSocket at:', wsProto + '//' + wsHost + '/ws/' + gameId);
        ws = new WebSocket(wsProto + '//' + wsHost + '/ws/' + gameId);

        ws.onmessage = function(event) {
            if (event.data === 'refresh') {
                const elapsed = Date.now() - lastMoveTime;
                if (elapsed < 1500) {
                    console.log('Suppressing WS refresh (local move POST update applied ' + elapsed + 'ms ago)');
                    return;
                }
                console.log('Received refresh signal from WS, reloading board');
                const urlParams = new URLSearchParams(window.location.search);
                const roleParam = urlParams.get('role') || '';
                htmx.ajax('GET', '/game/' + gameId + '/board?role=' + roleParam, {
                    target: document.getElementById('game-container'),
                    swap: 'innerHTML'
                });
            }
        };

        ws.onclose = function() {
            console.log('WS connection closed, retrying in 3 seconds...');
            setTimeout(connectWS, 3000);
        };

        ws.onerror = function(err) {
            console.error('WS error:', err);
        };
    }
    connectWS();

    // Add drag and drop listeners
    document.addEventListener('dragstart', function(e) {
        const boardEl = document.getElementById('chess-board');
        if (!boardEl) return;
        const myRole = boardEl.getAttribute('data-my-role');
        const myTurn = boardEl.getAttribute('data-my-turn') === 'true';
        if (myRole === 'spectator' || !myTurn) {
            e.preventDefault();
            return;
        }

        // Check if dragging board piece
        const squareEl = e.target.closest('[data-square]');
        if (squareEl) {
            const pieceColor = squareEl.getAttribute('data-piece-color');
            if (pieceColor !== myRole) {
                e.preventDefault();
                return;
            }
            const square = squareEl.getAttribute('data-square');
            e.dataTransfer.setData('text/plain', square);
            squareEl.classList.add('opacity-50');
            return;
        }

        // Check if dragging pocket piece
        const pocketEl = e.target.closest('[data-pocket-piece]');
        if (pocketEl) {
            const color = pocketEl.getAttribute('data-pocket-color');
            if (color !== myRole) {
                e.preventDefault();
                return;
            }
            const piece = pocketEl.getAttribute('data-pocket-piece');
            if (!piece) {
                e.preventDefault();
                return;
            }
            e.dataTransfer.setData('text/plain', 'drop:' + piece);
            pocketEl.classList.add('opacity-50');
            return;
        }

        e.preventDefault();
    });

    document.addEventListener('dragend', function(e) {
        const squareEl = e.target.closest('[data-square]');
        if (squareEl) {
            squareEl.classList.remove('opacity-50');
        }
        const pocketEl = e.target.closest('[data-pocket-piece]');
        if (pocketEl) {
            pocketEl.classList.remove('opacity-50');
        }
        document.querySelectorAll('.drag-over').forEach(el => el.classList.remove('drag-over'));
    });

    document.addEventListener('dragover', function(e) {
        const squareEl = e.target.closest('[data-square]');
        if (squareEl) {
            e.preventDefault();
        }
    });

    document.addEventListener('dragenter', function(e) {
        const squareEl = e.target.closest('[data-square]');
        if (squareEl) {
            squareEl.classList.add('drag-over');
        }
    });

    document.addEventListener('dragleave', function(e) {
        const squareEl = e.target.closest('[data-square]');
        if (squareEl) {
            squareEl.classList.remove('drag-over');
        }
    });

    document.addEventListener('drop', function(e) {
        const squareEl = e.target.closest('[data-square]');
        if (!squareEl) return;
        e.preventDefault();

        const boardEl = document.getElementById('chess-board');
        if (!boardEl) return;

        const myRole = boardEl.getAttribute('data-my-role');
        const gameId = boardEl.getAttribute('data-game-id');
        const toSq = squareEl.getAttribute('data-square');
        const fromSq = e.dataTransfer.getData('text/plain');

        if (!fromSq || fromSq === toSq) return;

        let promo = '';
        if (!fromSq.startsWith('drop:')) {
            let fromSquareEl = null;
            const sqs = document.querySelectorAll("[data-square]");
            for (let i = 0; i < sqs.length; i++) {
                if (sqs[i].getAttribute("data-square") === fromSq) {
                    fromSquareEl = sqs[i];
                    break;
                }
            }
            const isPawn = fromSquareEl && fromSquareEl.getAttribute('data-piece-type') === 'pawn';
            if (isPawn) {
                const toRank = toSq.charAt(1);
                if ((myRole === 'white' && toRank === '8') || (myRole === 'black' && toRank === '1')) {
                    const choice = prompt('Promote pawn to (Q = Queen, R = Rook, B = Bishop, N = Knight):', 'Q');
                    if (choice) {
                        promo = choice.trim().toLowerCase();
                    } else {
                        console.log('Promotion cancelled');
                        return;
                    }
                }
            }
        }

        const urlParams = new URLSearchParams(window.location.search);
        const roleParam = urlParams.get('role') || '';

        sendMoveRequest(gameId, roleParam, fromSq, toSq, promo);
    });

    document.addEventListener('click', function(e) {
        const boardEl = document.getElementById('chess-board');
        if (!boardEl) return;

        const myRole = boardEl.getAttribute('data-my-role');
        const myTurn = boardEl.getAttribute('data-my-turn') === 'true';

        if (myRole === 'spectator' || !myTurn) {
            return;
        }

        const pocketEl = e.target.closest('[data-pocket-piece]');
        if (pocketEl) {
            const piece = pocketEl.getAttribute('data-pocket-piece');
            const color = pocketEl.getAttribute('data-pocket-color');

            if (color !== myRole) {
                console.log('Cannot select opponent pocket piece');
                return;
            }

            const prevSelectedEl = document.querySelector('.selected-sq');
            if (prevSelectedEl) {
                prevSelectedEl.classList.remove('selected-sq');
            }
            const prevPocketEl = document.querySelector('.selected-pocket-piece');
            if (prevPocketEl) {
                prevPocketEl.classList.remove('selected-pocket-piece');
            }

            selectedSquare = 'drop:' + piece;
            selectedSquareEl = pocketEl;
            pocketEl.classList.add('selected-pocket-piece');
            console.log('Pocket piece selected:', selectedSquare);
            return;
        }

        const squareEl = e.target.closest('[data-square]');
        if (!squareEl) return;

        const square = squareEl.getAttribute('data-square');
        const pieceColor = squareEl.getAttribute('data-piece-color');

        if (selectedSquare === null) {
            if (pieceColor === myRole) {
                selectedSquare = square;
                selectedSquareEl = squareEl;
                squareEl.classList.add('selected-sq');
                console.log('Piece selected:', selectedSquare);
            } else {
                console.log('Cannot select opponent or empty square');
            }
        } else {
            if (selectedSquare === square || selectedSquare === ('drop:' + squareEl.getAttribute('data-piece-type'))) {
                if (selectedSquare.startsWith('drop:')) {
                    selectedSquareEl.classList.remove('selected-pocket-piece');
                } else {
                    selectedSquareEl.classList.remove('selected-sq');
                }
                selectedSquare = null;
                selectedSquareEl = null;
                console.log('Selection cleared');
            } else if (pieceColor === myRole) {
                if (selectedSquare.startsWith('drop:')) {
                    selectedSquareEl.classList.remove('selected-pocket-piece');
                } else {
                    selectedSquareEl.classList.remove('selected-sq');
                }
                selectedSquare = square;
                selectedSquareEl = squareEl;
                squareEl.classList.add('selected-sq');
                console.log('Selection changed to board piece:', selectedSquare);
            } else {
                const fromSq = selectedSquare;
                const toSq = square;
                const fromSquareEl = selectedSquareEl;

                if (selectedSquare.startsWith('drop:')) {
                    selectedSquareEl.classList.remove('selected-pocket-piece');
                } else {
                    selectedSquareEl.classList.remove('selected-sq');
                }
                selectedSquare = null;
                selectedSquareEl = null;

                let promo = '';
                if (!fromSq.startsWith('drop:')) {
                    const isPawn = fromSquareEl && fromSquareEl.getAttribute('data-piece-type') === 'pawn';
                    if (isPawn) {
                        const toRank = toSq.charAt(1);
                        if ((myRole === 'white' && toRank === '8') || (myRole === 'black' && toRank === '1')) {
                            const choice = prompt('Promote pawn to (Q = Queen, R = Rook, B = Bishop, N = Knight):', 'Q');
                            if (choice) {
                                promo = choice.trim().toLowerCase();
                            } else {
                                console.log('Promotion cancelled');
                                return;
                            }
                        }
                    }
                }

                const gameId = boardEl.getAttribute('data-game-id');
                const urlParams = new URLSearchParams(window.location.search);
                const roleParam = urlParams.get('role') || '';
                console.log('Submitting move/drop via HTMX AJAX:', fromSq, '->', toSq, 'promo:', promo);

                sendMoveRequest(gameId, roleParam, fromSq, toSq, promo);
            }
        }
    });
})();

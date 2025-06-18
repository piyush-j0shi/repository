call plug#begin('~/.vim/plugged')

" Color Schemes
Plug 'morhetz/gruvbox'
Plug 'joshdick/onedark.vim'
Plug 'folke/tokyonight.nvim'

" Rust support
Plug 'rust-lang/rust.vim'

" CoC (Autocomplete / LSP)
Plug 'neoclide/coc.nvim', {'branch': 'release'}

" Fuzzy finder
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'
Plug 'ctrlpvim/ctrlp.vim'

" File tree
Plug 'preservim/nerdtree'
Plug 'ryanoasis/vim-devicons'  " Icons!

" Status line
Plug 'vim-airline/vim-airline'
Plug 'vim-airline/vim-airline-themes'

" Git integration
Plug 'tpope/vim-fugitive'

" Rainbow parentheses
Plug 'luochen1990/rainbow'

" Start screen
Plug 'mhinz/vim-startify'

" Zen mode
Plug 'junegunn/goyo.vim'

" Cursor motion madness
Plug 'easymotion/vim-easymotion'

" Minimap
Plug 'wfxr/minimap.vim'

" Game to train Vim skills
Plug 'ThePrimeagen/vim-be-good'

" GitHub Copilot
Plug 'github/copilot.vim'

" Smooth scrolling + animation
Plug 'psliwka/vim-smoothie'
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'

call plug#end()

" Basic Setup
set nocompatible
syntax enable
filetype plugin indent on
set number
set relativenumber
set cursorline
set ruler
set scrolloff=5
set colorcolumn=69,420

" Toggle relative number on focus
augroup numbertoggle
  autocmd!
  autocmd BufEnter,FocusGained,InsertLeave * set relativenumber
  autocmd BufLeave,FocusLost,InsertEnter   * set norelativenumber
augroup END

" Theme
set background=dark
colorscheme gruvbox

" Rainbow parentheses
let g:rainbow_active = 1

" Lens + Animate smooth focus
let g:lens#animate = 1

" Smooth scrolling
" No config needed for vim-smoothie

" COC Config Section 
inoremap <silent><expr> <C-Space> coc#refresh()
inoremap <silent><expr> <Tab> coc#pum#visible() ? coc#pum#confirm() : "\<Tab>"
inoremap <silent><expr> <S-Tab> coc#pum#visible() ? coc#pum#select_prev() : "\<C-h>"
autocmd CursorHold * silent call CocActionAsync('highlight')

" Color popup menu (dark theme)
highlight Pmenu guibg=#1e1e1e guifg=#c0c0c0
highlight PmenuSel guibg=#264f78 guifg=#ffffff
highlight PmenuSbar guibg=#333333
highlight PmenuThumb guibg=#888888
highlight PmenuKind guifg=#87cefa
highlight PmenuExtra guifg=#aaaaaa

" === Copilot Setup ===
imap <silent><script><expr> <C-]> copilot#Accept("\<CR>")
let g:copilot_no_tab_map = v:true

" === Key Mappings ====
nnoremap <F2> :NERDTreeToggle<CR>
nnoremap <F3> :vert terminal<CR>
nnoremap <leader>z :Goyo<CR>
nmap s <Plug>(easymotion-s2)
nnoremap <leader>f :Files<CR>
nnoremap <leader>b :Buffers<CR>
nnoremap <leader>g :GFiles<CR>

" === Rust Settings ====
let g:rustfmt_autosave = 1

" === Startify (ASCII Startup) ==
let g:startify_custom_header = systemlist('figlet -f slant Vim Life')

" === Minimap Config === 
" set 0 if wants to close it
let g:minimap_auto_start = 1
let g:minimap_auto_start_win_enter = 1

" ASCII cow greetings
autocmd VimEnter * echo system('fortune | cowsay')

" Flashy intro (meme level)
set visualbell t_vb=
autocmd VimEnter * for i in range(1, 3) | exe "normal! \<Esc>" | sleep 10m | endfor

" GUI cursor animation
set guicursor=n-v-c:block-Cursor/lCursor-blinkon1

" Airline theme
let g:airline_theme='gruvbox'
let g:airline_powerline_fonts = 1

" Devicons support for NERDTree
let g:webdevicons_enable_nerdtree = 1
let g:NERDTreeShowIcons=1


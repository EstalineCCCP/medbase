syntax match MedbReferencia "^::.\{3,3}" 
hi def link MedbReferencia IncSearch
syntax match CorpoAncora /\v(^::...>)@<=.*/hs=s+1
highlight CorpoAncora ctermfg=150

syntax match PgNum "p\{1,2}\. \d\{1,5}\(\(,\|-\)\{0,1}\d\{1,5}\)\{0,5}"
hi def link PgNum Question

syntax match Etiqueta "\.\:\:.*\:\:"
highlight Etiqueta ctermfg=1 ctermbg=0

syntax match MedbComment "^#.*" 
highlight MedbComment ctermfg=245

syntax match Linque "http.*"
hi def link Linque Underlined

syntax match OBS "^OBS:\ "
hi def link OBS ErrorMsg
syntax match LinhaOBS /\v(OBS:\ )@<=.*/hs=s
highlight LinhaOBS ctermfg=245 gui=bold

set norelativenumber
set nonumber

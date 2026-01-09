[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, il widget dei commenti FastComments rileva automaticamente la modalità scura nella maggior parte dei siti.

Quando viene rilevata la modalità scura, FastComments passerà da testo nero su sfondo bianco a testo bianco su sfondo nero. Anche le immagini cambieranno.

Al caricamento della pagina, il widget cercherà di determinare quanto scuro sia lo sfondo della pagina dietro il widget dei commenti. Ciò significa che la pagina potrebbe avere uno sfondo bianco, ma se posizioni il widget dei commenti all'interno di un contenitore con sfondo nero, la modalità scura dovrebbe comunque attivarsi automaticamente per rendere i commenti leggibili.

Tuttavia, il meccanismo di rilevamento, che si basa sulla determinazione della "luminosità", potrebbe non attivare la modalità scura quando desideri. Per forzarla, imposta il flag *hasDarkBackground* su true come segue:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]
[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Antallet af kommentarer, der vises øverst i kommentar-widgetet, kan enten vise alle "top-level" kommentarer, altså de svar, som er direkte svar på siden eller artiklen selv, eller det kan være et tal for **alle** indlejrede kommentarer.

Som standard er dette `true` - det er et optælling af sidstnævnte - alle kommentarer. I ældre versioner af kommentar-widgetet var standardværdien `false`.

Vi kan ændre adfærden, så den tæller **alle** indlejrede kommentarer ved at sætte **countAll**-flaget til true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Hvis vi ønskede, at optællingen kun skulle afspejle topniveau-kommentarer, sætter vi flaget til false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Dette kan i øjeblikket ikke tilpasses uden kodeændringer.
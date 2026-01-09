[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Број коментара који се приказује на врху виджета за коментаре може да приказује или све „коментаре највишег нивоа“, што значи одговоре који
су директно упућени самој страници или чланку, или може бити број **свих** уроњених коментара.

By default, this is `true` - it is a count of the latter - all comments. In older versions of the comment widget the
default value is `false`.

We can change the behavior, so that it is a count of **all** nested comments by setting the **countAll** flag to true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

If we wanted the count to reflect only the top level comments, we set the flag to false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Ово тренутно не може бити прилагођено без измена кода.
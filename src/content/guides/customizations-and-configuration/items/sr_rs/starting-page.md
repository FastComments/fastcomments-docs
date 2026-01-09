[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

При преузимању и приказивању коментара, видгет за коментаре мора знати са које странице да почне. Подразумевано, он почиње са
прве странице и приказује само ту страницу.

Ако желите, тачно која страница треба да се прикаже може бити прослеђена видгету за коментаре као подешавање *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Имајте у виду да бројеви страница почињу од нуле, тако да горе наведен пример приказује другу страницу.
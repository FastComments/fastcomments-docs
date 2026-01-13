---
[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Подешавање `noNewRootComments` на `true` ће узроковати да виџет сакрије област за одговор на коренски коментар, али и даље омогући корисницима да одговарају на подређене коментаре. На пример, можете ово условно поставити при учитавању странице да бисте само неким корисницима дозволили да остављају коментаре на највишем нивоу.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---
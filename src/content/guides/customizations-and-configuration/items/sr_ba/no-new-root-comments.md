[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Постављање `noNewRootComments` на `true` ће узроковати да видгет сакрије поље за одговарање на коренском нивоу, али ће и даље дозволити корисницима да одговарају
на подкоментаре. На пример, можете ово условно поставити при учитавању странице да бисте омогућили само неким корисницима да остављају коментаре највишег нивоа.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---
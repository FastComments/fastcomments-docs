[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Подешавање `noNewRootComments` на `true` ће узроковати да видгет сакрије главно поље за одговор, али ће и даље омогућити корисницима да одговарају на подкоментаре. На пример, можете ово условно поставити при учитавању странице да бисте само неким корисницима дозволили да остављају коментаре највишег нивоа.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]
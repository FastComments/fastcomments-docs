---
[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments дозвољава корисницима да отпремају слике. Када корисник кликне на ту слику, FastComments ће, подразумевано,
отворити нову картицу да би приказао слику у пуној величини. Постављање ове ознаке на true онемогућава то понашање:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Ако не намеравате сами да обрађујете кликове на слику (погледајте [onImageClicked](#callbacks)), препоручујемо да се ово комбинује са одговарајућим стиловањем
које уклања визуелни утисак да је слика кликабилна.

---
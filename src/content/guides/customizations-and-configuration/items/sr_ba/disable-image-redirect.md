---
[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments дозвољава корисницима да отпремају слике. Када корисник кликне на ту слику, FastComments ће, подразумевано, отворити нову картицу да прикаже ту слику у пуној величини. Постављање ове опције на true онемогућава ово понашање:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Ако не планирате сами да обрадите кликање на слику (погледајте [onImageClicked](#callbacks)), препоручујемо да се ово комбинује са неким стиловањем како би се уклонио утисак да је слика кликабилна.

---
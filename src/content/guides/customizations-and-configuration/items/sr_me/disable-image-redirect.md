[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

По подразумевању, FastComments омогућава корисницима да отпремају слике. Када корисник кликне ту слику, FastComments ће, по подразумевању,
отвори нову картицу да прикаже ту слику у пуној величини. Постављање ове заставице на true онемогућава ово понашање:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Ако не планирате да сами ухватите клик на слику (погледајте [onImageClicked](#callbacks)), препоручујемо да се ово комбинује са неком стилизацијом
како бисте уклонили изглед да се на слику може кликнути.
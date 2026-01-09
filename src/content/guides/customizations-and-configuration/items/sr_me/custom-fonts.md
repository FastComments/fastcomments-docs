[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments је дизајниран да буде прилагођен, и фонт који користе наши видџети није изузетак.

По подразумевању, FastComments користи `system font stack` да би изгледао што боље на широком спектру уређаја.

Да бисте дефинисали своје фонтове, погледајте [Документацију о прилагођеном CSS-у](/guide-customizations-and-configuration.html#custom-css).

Тамо ћете пронаћи начин да дефинишете прилагођени CSS, што ће вам омогућити да одредите жељене фонтове.

#### Како дефинисати фонт

Да бисте променили фонт, препоручујемо да дефинишете ваш CSS користећи селекторе `.fast-comments, textarea`. На пример:

[inline-code-attrs-start title = 'Пример прилагођеног спољашњег фонта'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---
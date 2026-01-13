[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments је дизајниран да буде прилагодљив, и фонт који наши видгети користе није изузетак.

По подразумевaњу, FastComments користи `system font stack` да би изгледао што боље на широком спектру уређаја.

Да бисте дефинисали своје фонтове, погледајте [Документација за прилагођени CSS](/guide-customizations-and-configuration.html#custom-css).

Тамо ћете наћи начин да дефинишете прилагођени CSS, што ће вам омогућити да одредите жељене фонтове.

#### Како дефинисати фонт

Да бисте заменили фонт, препоручујемо да дефинишете свој CSS користећи селекторе `.fast-comments, textarea`. На пример:

[inline-code-attrs-start title = 'Примјер прилагођеног спољног фонта'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---
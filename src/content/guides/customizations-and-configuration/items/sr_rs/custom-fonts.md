[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments је дизајниран да се прилагоди, и фонт који наши видгети користе није изузетак.

Подразумевано, FastComments користи `system font stack` како би изгледао што боље на широком спектру уређаја.

Да бисте дефинисали сопствене фонтове, погледајте [Документација о прилагођеном CSS-у](/guide-customizations-and-configuration.html#custom-css).

Тамо ћете пронаћи начин да дефинишете прилагођени CSS, који ће вам омогућити да подесите жељене фонтове.

#### Како дефинисати фонт

Да бисте заменили фонт, препоручујемо да дефинишете ваш CSS користећи селекторе `.fast-comments, textarea`. На пример:

[inline-code-attrs-start title = 'Пример прилагођеног спољног фонта'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---
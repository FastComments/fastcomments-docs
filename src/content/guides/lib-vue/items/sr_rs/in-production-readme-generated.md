Вероватно не желите да дефинишете config инлајн ако прослеђујете callbacks итд. Уместо тога, желећете да дефинишете
config у `computed` блоку, иначе ће се сваки пут када се ваш callback итд позове цео widget поново рендеровати.

[Погледајте пример spinner-а да видите како се то ради.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
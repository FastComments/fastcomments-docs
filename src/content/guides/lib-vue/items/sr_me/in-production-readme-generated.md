Вероватно не желите да дефинишете config инлајн ако прослеђујете колбеке итд. Уместо тога, желећете да дефинишете
config у `computed` блоку, иначе ће сваки пут када се ваш колбек итд позове цео widget бити поново рендерован.

[Погледајте пример spinner-а да видите како се то ради.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
Вероятно не искате да дефинирате config inline, ако предавате callbacks и т.н. Вместо това ще искате да дефинирате
config в `computed` блок, иначе всеки път когато вашият callback и т.н. бъде извикан, целият widget ще се рендерира отново.

[Вижте примера със spinner за това как да го направите.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
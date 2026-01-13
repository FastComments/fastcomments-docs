Je wilt de config waarschijnlijk niet inline definiëren als je callbacks enz. doorgeeft. In plaats daarvan wil je de config in een `computed`-blok definiëren; anders zal elke keer dat je callback enz. wordt aangeroepen, het hele widget opnieuw renderen.

[Bekijk het spinnervoorbeeld om te zien hoe je dit doet.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
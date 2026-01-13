Verjetno ne želite definirati config inline, če posredujete callbacks itd. Namesto tega boste želeli definirati
config v `computed` bloku, sicer se bo vsakič, ko bo vaš callback itd. poklican, celoten widget znova upodobil.

[Oglejte si primer spinnerja, kako to narediti.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
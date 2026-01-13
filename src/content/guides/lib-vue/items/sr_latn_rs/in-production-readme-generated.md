Verovatno ne želite da definišete config inline ako prosleđujete callbacks itd. Umesto toga, želećete da definišete
config u `computed` bloku, inače će se ceo widget svaki put kad se vaš callback itd. pozove ponovo renderovati.

[Pogledajte primer spinner-a kako to uraditi.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
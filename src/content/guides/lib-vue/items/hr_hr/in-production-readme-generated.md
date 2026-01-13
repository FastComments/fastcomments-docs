Vjerojatno ne želite definirati konfiguraciju inline ako prosljeđujete povratne pozive itd. Umjesto toga, želite definirati
konfiguraciju u `computed` bloku, inače će se svaki put kada se vaš povratni poziv itd. pozove cijeli widget ponovno renderirati.

[Pogledajte primjer spinnera kako to učiniti.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
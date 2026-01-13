Du vil sandsynligvis ikke ønske at definere config inline, hvis du videregiver callbacks osv. I stedet vil du definere
config i en `computed`-blok, ellers vil hele widget'en blive gengivet på ny hver gang din callback osv. bliver kaldt.

[Se spinner-eksemplet for hvordan du gør dette.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
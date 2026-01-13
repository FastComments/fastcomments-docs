Prawdopodobnie nie chcesz definiować konfiguracji inline, jeśli przekazujesz callbacki itp. Zamiast tego będziesz chciał zdefiniować
konfigurację w bloku `computed`, w przeciwnym razie za każdym razem, gdy twój callback itp. zostanie wywołany, cały widżet zostanie ponownie wyrenderowany.

[Zobacz przykład spinnera, aby dowiedzieć się, jak to zrobić.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
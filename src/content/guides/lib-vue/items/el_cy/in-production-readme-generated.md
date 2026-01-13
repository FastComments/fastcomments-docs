Μάλλον δεν θέλετε να ορίσετε το config inline αν περνάτε callbacks κ.λπ. Αντίθετα, θα θέλετε να ορίσετε
το config σε ένα `computed` block, διαφορετικά κάθε φορά που το callback κ.λπ. καλείται ολόκληρο το widget θα ξανα-αποδοθεί.

[Δείτε το παράδειγμα spinner για το πώς να το κάνετε.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
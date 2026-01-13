Πιθανότατα δεν θέλετε να ορίσετε το config ενσωματωμένα αν περνάτε callbacks κ.λπ. Αντίθετα, θα θελήσετε να ορίσετε
το config σε ένα μπλοκ `computed`, αλλιώς κάθε φορά που καλείται ο callback σας κ.λπ. ολόκληρο το widget θα re-render.

[Δείτε το παράδειγμα spinner για το πώς να το κάνετε.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)
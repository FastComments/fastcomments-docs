[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Για ιστότοπους που επιτρέπουν την εναλλαγή σκοτεινής λειτουργίας μετά την αρχική φόρτωση της σελίδας, αυτό είναι λίγο πιο περίπλοκο.

Καταρχάς, όλες οι τρέχουσες εκδόσεις της βιβλιοθήκης widget Σχολίων (React, Vue) περιέχουν παραδείγματα εναλλαγής σκοτεινής λειτουργίας στα αντίστοιχα αποθετήριά τους.

Για το widget VanillaJS, θα χρειαστεί να κάνουμε λίγη περισσότερη δουλειά. Αρχικά, το FastCommentsUI επιστρέφει ένα αντικείμενο με τις συναρτήσεις "destroy" και "update".

Μπορούμε απλά να καλέσουμε τη συνάρτηση update κάθε φορά που θέλουμε να ενημερώσουμε τη διαμόρφωση του widget σχολίων, ως εξής. Εδώ είναι ένα πλήρες λειτουργικό παράδειγμα εναλλαγής
σκοτεινής λειτουργίας με το widget VanillaJS.

[inline-code-attrs-start title = 'Πλήρες Παράδειγμα Εναλλαγής Σκοτεινής Λειτουργίας'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---
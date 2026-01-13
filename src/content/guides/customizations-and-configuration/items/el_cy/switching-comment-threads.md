[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Έχουμε εξηγήσει πως το `urlId` είναι το αναγνωριστικό της σελίδας ή του άρθρου στο οποίο συνδέονται τα σχόλια.

Επίσης, για επανάληψη, αν δεν οριστεί, το `urlId` θα έχει προεπιλογή το URL της τρέχουσας σελίδας.

Τι γίνεται με τις εφαρμογές μίας σελίδας (SPAs), όπου η σελίδα ή το περιεχόμενο στο οποίο συνδέονται τα σχόλια αλλάζει δυναμικά χωρίς πλήρη επαναφόρτωση της σελίδας;

#### Angular, React, Vue, κ.λπ.

Με τις βιβλιοθήκες μας όπως οι Angular και React, το απλό ενημέρωμα της ιδιότητας `urlId` που δίνεται στο widget θα προκαλέσει ανανέωση του widget σχολίων. Μπορείτε να δείτε αυτό σε δράση για την εφαρμογή React, για παράδειγμα, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">εδώ</a>.

#### VanillaJS

Αν χρησιμοποιείτε τη βιβλιοθήκη VanillaJS είναι ελαφρώς πιο περίπλοκο καθώς δεν υπάρχει ένα πλαίσιο όπως ο Angular ή ο React για να χειριστεί το binding δεδομένων ή τη διάδοση κατάστασης.

Όταν δημιουργείτε μια παρουσία του widget VanillaJS, επιστρέφει μερικές συναρτήσεις που μπορούν να κληθούν για να το ενημερώσουν.

Ιδού ένα λειτουργικό παράδειγμα όπου αλλάζουμε το hash της σελίδας και ενημερώνουμε το widget σχολίων:

[inline-code-attrs-start title = 'Παράδειγμα αλλαγής hash σελίδας'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---
[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Έχουμε δει πώς το `urlId` είναι το αναγνωριστικό της σελίδας ή του άρθρου στο οποίο είναι συνδεδεμένα τα σχόλια.

Επίσης, για να το επαναλάβουμε, αν δεν οριστεί, το `urlId` θα πάρει ως προεπιλογή το URL της τρέχουσας σελίδας.

Τι γίνεται όμως με τα SPAs, ή Single-Page-Applications, όπου η σελίδα ή το περιεχόμενο στο οποίο είναι συνδεδεμένα τα σχόλια αλλάζει
δυναμικά χωρίς νέα επαναφόρτωση της σελίδας;

#### Angular, React, Vue, κ.λπ.

Με τις βιβλιοθήκες μας όπως το Angular και το React, η απλή ενημέρωση της ιδιότητας `urlId` που περνάει στο widget
θα προκαλέσει την ανανέωση του widget σχολίων. Μπορείτε να το δείτε σε δράση για την εφαρμογή React, για παράδειγμα, <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">εδώ</a>.

#### VanillaJS

Αν χρησιμοποιείτε τη βιβλιοθήκη VanillaJS, τα πράγματα είναι λίγο πιο περίπλοκα, καθώς δεν υπάρχει ένα framework όπως το Angular ή το React
που να χειρίζεται το data binding ή τη διάδοση της κατάστασης.

Όταν δημιουργείτε το widget VanillaJS, επιστρέφει κάποιες συναρτήσεις που μπορούν να κληθούν για να το ενημερώσουν.

Ακολουθεί ένα λειτουργικό παράδειγμα όπου αλλάζουμε το hash της σελίδας και ενημερώνουμε το widget σχολίων:

[inline-code-attrs-start title = 'Παράδειγμα Αλλαγής Hash Σελίδας'; inline-code-attrs-end]
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

            config.url = locationString; // Ενημερώνουμε και το url, ώστε οι ειδοποιήσεις να συνδέονται με τη σωστή σελίδα
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]
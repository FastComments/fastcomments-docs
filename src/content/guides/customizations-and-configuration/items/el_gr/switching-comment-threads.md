[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Έχουμε καλύψει πώς το `urlId` είναι το αναγνωριστικό (id) της σελίδας ή του άρθρου στο οποίο συνδέονται τα σχόλια.

Επίσης, για σύνοψη, εάν δεν οριστεί, το `urlId` θα οριστεί από προεπιλογή στο τρέχον URL της σελίδας.

Τι γίνεται με τις SPA, ή Single-Page-Applications, όπου η σελίδα ή το περιεχόμενο στο οποίο συνδέονται τα σχόλια αλλάζει
δυναμικά χωρίς επαναφόρτωση της σελίδας;

#### Angular, React, Vue, etc

Με τις βιβλιοθήκες μας όπως οι Angular και React, απλώς η ενημέρωση της ιδιότητας `urlId` που περνάει στο widget
θα προκαλέσει την ανανέωση του widget σχολίων. Μπορείτε να δείτε αυτό σε δράση για την εφαρμογή React, για παράδειγμα, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">εδώ</a>.

#### VanillaJS

Αν χρησιμοποιείτε τη βιβλιοθήκη VanillaJS, είναι λίγο πιο περίπλοκο καθώς δεν υπάρχει ένα πλαίσιο όπως το Angular ή το React
για να χειρίζεται τη δεσμοποίηση δεδομένων ή τη διάδοση κατάστασης.

Όταν δημιουργείτε μια παρουσία του widget VanillaJS, επιστρέφει κάποιες συναρτήσεις οι οποίες μπορούν να καλούνται για να το ενημερώσουν.

Ακολουθεί ένα λειτουργικό παράδειγμα όπου αλλάζουμε το hash της σελίδας και ενημερώνουμε το widget σχολίων:

[inline-code-attrs-start title = 'Παράδειγμα αλλαγής του hash της σελίδας'; inline-code-attrs-end]
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

            config.url = locationString; // Ενημερώνουμε επίσης το url, ώστε οι ειδοποιήσεις να μπορούν να συνδέονται πίσω στη σωστή σελίδα
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---
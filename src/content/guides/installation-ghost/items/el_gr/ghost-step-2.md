---
Τώρα που κατεβάσαμε το αρχείο zip, αποσυμπιέστε το σε έναν φάκελο. Έχω κατεβάσει το προεπιλεγμένο `casper.zip` και το αποσυμπίεσα στο `Downloads\casper` στα Windows.

Στη συνέχεια, βεβαιωθείτε ότι έχετε εγκατεστημένη την LTS ή νεότερη έκδοση του NodeJS. Μπορείτε να την κατεβάσετε εδώ: https://nodejs.org/en/download/

Μόλις εγκατασταθεί το NodeJS, θα πρέπει να εγκαταστήσετε έναν επεξεργαστή κώδικα.

Συστήνουμε (και χρησιμοποιούμε) το Webstorm, το οποίο μπορείτε να αποκτήσετε εδώ με δοκιμή 30 ημερών (δεν απαιτείται πιστωτική κάρτα): https://www.jetbrains.com/webstorm/

Η επόμενη καλύτερη δωρεάν επιλογή πιθανώς είναι το Visual Studio Code: https://code.visualstudio.com/download

Αφού ρυθμίσετε τον επεξεργαστή σας και ανοίξετε τον φάκελο του θέματος στον επεξεργαστή, ανοίξτε το τερματικό στο IDE και εκτελέστε:

[inline-code-attrs-start title = 'Εγκατάσταση του Θέματος'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Η επιτυχής έξοδος θα μοιάζει με αυτή (μπορείτε να αγνοήσετε τις προειδοποιήσεις):

<div class="screenshot white-bg">
    <div class="title">Επιτυχής έξοδος npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Επιτυχής έξοδος npm install" />
</div>

Αυτό θα ρυθμίσει τις εξαρτήσεις του θέματος για τις επόμενες εντολές που θα εκτελέσουμε. Επίσης, η εξαγωγή εξαρτάται από το να είναι εγκατεστημένες οι εξαρτήσεις του θέματος, διαφορετικά η επανεισαγωγή δεν θα λειτουργήσει σωστά.

---
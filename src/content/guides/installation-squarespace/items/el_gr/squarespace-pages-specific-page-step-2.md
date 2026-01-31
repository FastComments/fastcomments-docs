Τώρα μπορούμε να αντιγράψουμε το ακόλουθο απόσπασμα κώδικα. Χρησιμοποιήστε το κουμπί αντιγραφής που εμφανίζεται πάνω δεξιά στο απόσπασμα.

Υπάρχουν μερικά πράγματα που μπορείτε να διαμορφώσετε στον κώδικα, δείτε τις γραμμές 4 έως 7.

[inline-code-attrs-start title = 'Κώδικας Μίας Σελίδας Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // το αναγνωριστικό του λογαριασμού σας
    }];
</script>
[inline-code-end]

It should look like this:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>

Now click save in the top right.

Note that the `Preview in Safe Mode` option will not work, but the widget will appear when you visit your site.

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.
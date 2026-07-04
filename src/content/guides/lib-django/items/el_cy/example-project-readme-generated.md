Μία εκτελέσιμη επίδειξη βρίσκεται στο [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): μια εφαρμογή με αριστερό rail + κύριο στάδιο, με μια σελίδα ανά widget και μια **σελίδα σύνδεσης που εμφανίζει προ‑σπείρωμένους demo χρήστες**. Συνδεθείτε με οποιονδήποτε από αυτούς και τα widgets σχολίων και ζωντανής συνομιλίας επαληθεύουν αυτή την ταυτότητα μέσω **Secure SSO**. Από αυτόν τον φάκελο:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Χωρίς ένα API secret επιστρέφει στον δημόσιο `demo` tenant (ανώνυμο). Το [`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) είναι ένα Playwright e2e που φορτώνει τη σελίδα σε πραγματικό πρόγραμμα περιήγησης και δημοσιεύει ένα σχόλιο ως ο χρήστης Secure‑SSO.
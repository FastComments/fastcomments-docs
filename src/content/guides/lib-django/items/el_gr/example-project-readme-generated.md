Μια εκτελέσιμη παρουσίαση βρίσκεται στο [`example/`](https://github.com/FastComments/fastcomments-django/blob/main/example): μια εφαρμογή με αριστερή μπάρα + κύριο σκηνικό με μια σελίδα ανά widget και μια **σελίδα σύνδεσης που εμφανίζει προ-σπόρους χρήστες demo**.  
Συνδεθείτε με οποιονδήποτε από αυτούς και τα widgets σχολίων και ζωντανής συνομιλίας επαληθεύουν αυτήν την ταυτότητα μέσω **Secure SSO**. Από αυτόν τον φάκελο:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

Χωρίς μυστικό API επιστρέφει στον δημόσιο `demo` ενοικιαστή (ανώνυμο).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) είναι ένα Playwright e2e που φορτώνει τη σελίδα σε πραγματικό πρόγραμμα περιήγησης και στέλνει ένα σχόλιο ως ο χρήστης Secure-SSO.
Το FastComments λειτουργεί έναν φιλοξενούμενο διακομιστή Model Context Protocol (MCP), ώστε οι AI βοηθοί και οι agentic πελάτες να μπορούν να καλούν απευθείας το FastComments API. Κάθε εργαλείο που εκθέτει ο διακομιστής MCP δημιουργείται αυτόματα από τη δημόσια προδιαγραφή OpenAPI, οπότε ό,τι μπορεί να κάνει το REST API, μπορεί να το κάνει και ένας πελάτης MCP.

Το endpoint είναι stateless και βασίζεται σε streamable-HTTP. Δεν υπάρχει συνεδρία που να χρειάζεται διατήρηση ούτε server-side κατάσταση ανά πελάτη.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Σύνδεση με OAuth

Οποιοσδήποτε πελάτης MCP που υποστηρίζει απομακρυσμένους διακομιστές με OAuth (Claude, ChatGPT, Claude Code, Cursor και άλλοι) μπορεί να συνδεθεί στο παραπάνω endpoint χωρίς καμία ρύθμιση από την πλευρά του FastComments. Ο πελάτης εγγράφεται μέσω Dynamic Client Registration ή ταυτοποιείται με ένα Client ID Metadata Document, ανοίγει ένα πρόγραμμα περιήγησης ώστε να συνδεθείτε στο FastComments και να εγκρίνετε την πρόσβαση, και λαμβάνει ένα token δεσμευμένο στον λογαριασμό στον οποίο ήσασταν συνδεδεμένοι.

Τα έγγραφα discovery βρίσκονται στις τυπικές τοποθεσίες:

[inline-code-attrs-start title = 'Discovery'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Ο χρήστης σας χρειάζεται την άδεια API Admin στον λογαριασμό για να εγκρίνει μια σύνδεση. Αν διαχειρίζεστε πολλούς λογαριασμούς, μεταβείτε στον σωστό στο dashboard πριν εγκρίνετε.

Ένας πελάτης μπορεί να ζητήσει το scope `read`, το scope `write` ή και τα δύο. Ένας πελάτης που δεν ζητά τίποτα λαμβάνει και τα δύο. Εργαλεία που τροποποιούν δεδομένα δεν προσφέρονται σε token μόνο ανάγνωσης.

Το dashboard διαθέτει έναν βοηθό ρύθμισης με έτοιμα αποσπάσματα κώδικα για επικόλληση. Ανοίξτε **Integrate -> MCP Server**, ή επισκεφθείτε το απευθείας:

[inline-code-attrs-start title = 'Σελίδα Ρύθμισης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Καταχωρήστε τον διακομιστή FastComments με μία εντολή και, στη συνέχεια, εκτελέστε `/mcp` μέσα σε μια συνεδρία για να συνδεθείτε και να δείτε τα διαθέσιμα εργαλεία:

[inline-code-attrs-start title = 'Ρύθμιση Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor και άλλοι πελάτες με αρχείο ρυθμίσεων

Προσθέστε αυτό το μπλοκ στη ρύθμιση MCP servers του πελάτη σας (`mcp.json` για το Cursor). Ο πελάτης ανοίγει ένα πρόγραμμα περιήγησης για σύνδεση κατά την πρώτη χρήση.

[inline-code-attrs-start title = 'Ρύθμιση Πελάτη MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Ανάκληση πρόσβασης

Κάθε εγκεκριμένη σύνδεση εμφανίζεται κάτω από **Integrate -> Connected Apps** στο dashboard. Η ανάκληση μιας σύνδεσης ακυρώνει κάθε token που κατέχει η εφαρμογή. Οι εφαρμογές εγγράφονται μόνες τους όταν συνδέονται και το FastComments δεν τις ελέγχει, οπότε ανακαλέστε ό,τι δεν αναγνωρίζετε.

### Χρήση του token με το REST API

Το access token που αποκτά ένας πελάτης MCP είναι ένα κανονικό διαπιστευτήριο του FastComments API. Λειτουργεί σε κάθε endpoint `/api/v1` ως bearer token, οπότε μια εφαρμογή που συνδέθηκε μέσω MCP μπορεί επίσης να καλεί απευθείας το REST API:

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Ο tenant συνάγεται από το token. Ένα `tenantId` μπορεί ακόμα να περαστεί, αλλά πρέπει να ταιριάζει. Τα αιτήματα `GET` χρειάζονται το scope `read` και όλα τα υπόλοιπα το scope `write`.

### Σύνδεση με κλειδί API

Πελάτες που δεν μπορούν να ολοκληρώσουν σύνδεση μέσω προγράμματος περιήγησης, όπως διακομιστές χωρίς γραφικό περιβάλλον, μπορούν να πιστοποιηθούν με κλειδί API. Περάστε `tenantId` και `API_KEY` ως παραμέτρους query, ή ως HTTP headers `x-tenant-id` και `x-api-key` αν ο πελάτης σας υποστηρίζει προσαρμοσμένα headers:

[inline-code-attrs-start title = 'Endpoint με Κλειδί API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Η σελίδα ρύθμισης δημιουργεί αυτό το URL για κάθε ένα από τα κλειδιά API σας.

### Ασφάλεια

Ένα URL endpoint που περιέχει κλειδί API είναι μυστικό: μην το επικολλάτε σε δημόσιες συνομιλίες, στιγμιότυπα οθόνης ή commits. Αν ένα κλειδί εκτεθεί, ανανεώστε το στη σελίδα API Keys στο dashboard σας. Τα OAuth tokens δεν ενέχουν τέτοιον κίνδυνο, επειδή είναι δεσμευμένα σε μία εφαρμογή και μπορούν να ανακληθούν από τα Connected Apps.
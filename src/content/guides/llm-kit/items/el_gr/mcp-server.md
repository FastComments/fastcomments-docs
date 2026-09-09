FastComments εκτελεί έναν φιλοξενούμενο Model Context Protocol (MCP) ώστε οι βοηθοί AI και οι πράκτορες-πελάτες να μπορούν να καλούν απευθείας το API της FastComments. Κάθε εργαλείο που εκθέτει ο διακομιστής MCP δημιουργείται αυτόματα από το δημόσιο OpenAPI spec, έτσι ό,τι μπορεί να κάνει το REST API, μπορεί να κάνει και ένας πελάτης MCP.

Το endpoint είναι αstateless και βασίζεται σε streamable-HTTP. Δεν υπάρχει συνεδρία που πρέπει να διατηρηθεί ζωντανή και δεν υπάρχει κατάσταση στο διακομιστή ανά πελάτη.

### Endpoint

[inline-code-attrs-start title = 'Τελικό Σημείο MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Οποιοσδήποτε πελάτης MCP που υποστηρίζει απομακρυσμένους διακομιστές με OAuth (Claude, ChatGPT, Claude Code, Cursor και άλλοι) μπορεί να συνδεθεί στο παραπάνω endpoint χωρίς καμία ρύθμιση από την πλευρά της FastComments. Ο πελάτης εγγράφεται μέσω Dynamic Client Registration ή ταυτοποιείται με ένα Client ID Metadata Document, ανοίγει έναν φυλλομετρητή ώστε να συνδεθείτε στη FastComments και να εγκρίνετε την πρόσβαση, και λαμβάνει ένα token που συνδέεται με τον λογαριασμό στον οποίο ήσασταν συνδεδεμένοι.

Τα έγγραφα discovery βρίσκονται στις τυπικές τοποθεσίες:

[inline-code-attrs-start title = 'Ανακάλυψη'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Ο χρήστης σας χρειάζεται την άδεια API Admin στον λογαριασμό για να εγκρίνει μια σύνδεση. Εάν διαχειρίζεστε πολλούς λογαριασμούς, μεταβείτε στον σωστό στο dashboard πριν εγκρίνετε.

Ένας πελάτης μπορεί να ζητήσει το πεδίο `read`, το πεδίο `write`, ή και τα δύο. Ένας πελάτης που δεν ζητά τίποτα λαμβάνει και τα δύο. Τα εργαλεία που αλλάζουν δεδομένα δεν προσφέρονται σε token μόνο για ανάγνωση.

Το dashboard διαθέτει έναν βοηθό ρύθμισης με έτοιμα αποσπάσματα προς επικόλληση. Ανοίξτε **Integrate -> MCP Server**, ή επισκεφθείτε το απευθείας:

[inline-code-attrs-start title = 'Σελίδα Ρύθμισης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Καταχωρίστε τον διακομιστή FastComments με μία εντολή, έπειτα εκτελέστε `/mcp` μέσα σε μια συνεδρία για να συνδεθείτε και να εμφανίσετε τα διαθέσιμα εργαλεία:

[inline-code-attrs-start title = 'Ρύθμιση Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Προσθέστε αυτό το μπλοκ στη διαμόρφωση MCP servers του πελάτη σας (`mcp.json` για το Cursor). Ο πελάτης ανοίγει έναν φυλλομετρητή για να συνδεθεί στην πρώτη χρήση.

[inline-code-attrs-start title = 'Διαμόρφωση Πελάτη MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Revoking access

Κάθε εγκεκριμένη σύνδεση εμφανίζεται κάτω από **Integrate -> Connected Apps** στο dashboard. Η ανάκληση μιας σύνδεσης ακυρώνει κάθε token που κατέχει η εφαρμογή. Οι εφαρμογές εγγράφονται όταν συνδέονται και η FastComments δεν τις ελέγχει, επομένως ανακαλέστε ό,τι δεν αναγνωρίζετε.

### Using the token with the REST API

Το access token που λαμβάνει ένας πελάτης MCP είναι ένα κανονικό διαπιστευτήριο API της FastComments. Λειτουργεί σε κάθε endpoint `/api/v1` ως bearer token, έτσι μια εφαρμογή που συνδέθηκε μέσω MCP μπορεί επίσης να καλέσει απευθείας το REST API:

[inline-code-attrs-start title = 'Token Φορέα'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Το tenant προκύπτει από το token. Ένα `tenantId` μπορεί ακόμη να περαστεί αλλά πρέπει να ταιριάζει. Τα αιτήματα `GET` χρειάζονται το πεδίο `read` και όλα τα άλλα χρειάζονται το πεδίο `write`.

### Connect with an API key

Οι πελάτες που δεν μπορούν να ολοκληρώσουν σύνδεση μέσω φυλλομετρητή, όπως headless servers, μπορούν να πιστοποιηθούν με ένα API key. Περνάτε το `tenantId` και το `API_KEY` ως παραμέτρους ερωτήματος, ή ως τις HTTP κεφαλίδες `x-tenant-id` και `x-api-key` εάν ο πελάτης σας υποστηρίζει προσαρμοσμένες κεφαλίδες:

[inline-code-attrs-start title = 'Τελικό Σημείο API Key'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Η σελίδα ρύθμισης δημιουργεί αυτό το URL για κάθε ένα από τα API keys σας.

### Security

Ένα URL endpoint που περιέχει API key είναι μυστικό: μην το επικολλάτε σε δημόσιες συνομιλίες, στιγμιότυπα οθόνης ή commits. Εάν εκτεθεί ένα κλειδί, ανανεώστε το στη σελίδα API Keys στο dashboard σας. Τα OAuth tokens δεν φέρουν τέτοιο κίνδυνο επειδή συνδέονται με μία εφαρμογή και μπορούν να ανακληθούν από Connected Apps.
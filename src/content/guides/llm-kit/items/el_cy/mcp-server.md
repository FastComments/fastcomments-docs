FastComments τρέχει έναν φιλοξενούμενο εξυπηρετητή Model Context Protocol (MCP) ώστε βοηθοί κωδικοποίησης με AI και αυτόνομοι πελάτες να μπορούν να καλούν απευθείας το FastComments API. Κάθε εργαλείο που εκθέτει ο εξυπηρετητής MCP παραγίνεται αυτόματα από το δημόσιο OpenAPI spec, οπότε οτιδήποτε μπορεί να κάνει το REST API, μπορεί να το κάνει και ένας MCP client.

Το τελικό σημείο είναι χωρίς κατάσταση (stateless) και βασισμένο σε streamable-HTTP. Δεν υπάρχει συνεδρία για να κρατήσετε ζωντανή, κανένα βήμα εγγραφής πελάτη, και κανένα server-side state ανά πελάτη.

### Τελικό Σημείο

[inline-code-attrs-start title = 'Τελικό Σημείο MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Η αυθεντικοποίηση χρησιμοποιεί το ίδιο API key με το REST API. Μπορείτε επίσης να περάσετε το `tenantId` και το κλειδί ως HTTP headers `x-tenant-id` και `x-api-key` εάν ο client σας υποστηρίζει προσαρμοσμένα headers.

### Προ-συμπληρωμένη ρύθμιση

Ο πίνακας ελέγχου έχει έναν οδηγό ρύθμισης που δημιουργεί το URL και έτοιμα προς επικόλληση αποσπάσματα διαμόρφωσης για δημοφιλείς MCP clients. Πηγαίνετε στον πίνακα ελέγχου του λογαριασμού σας και ανοίξτε **Integrate -> MCP Server**, ή επισκεφθείτε το απευθείας:

[inline-code-attrs-start title = 'Σελίδα Ρύθμισης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Επιλέξτε ποιο API key θα χρησιμοποιήσετε από το dropdown, και στη συνέχεια αντιγράψτε οποιοδήποτε από τα γεννημένα αποσπάσματα.

### Claude Code

Καταχωρήστε τον εξυπηρετητή FastComments με μία εντολή:

[inline-code-attrs-start title = 'Ρύθμιση Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Αφού καταχωρηθεί, τρέξτε `/mcp` μέσα σε ένα session του Claude Code για να επιβεβαιώσετε τη σύνδεση και να δείτε τη λίστα με τα διαθέσιμα εργαλεία.

### Claude Desktop / Cursor

Προσθέστε αυτό το μπλοκ στη διαμόρφωση MCP servers του client σας (`claude_desktop_config.json` για Claude Desktop, `mcp.json` για Cursor):

[inline-code-attrs-start title = 'Διαμόρφωση Πελάτη MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### Ασφάλεια

Το API key είναι ενσωματωμένο στο URL. Αντιμετωπίστε το URL σαν μυστικό: μην το επικολλάτε σε δημόσιες συζητήσεις, στιγμιότυπα οθόνης ή commits. Εάν ένα κλειδί εκτεθεί, αντικαταστήστε/περιστρέψτε το στη σελίδα Κλειδιών API στον πίνακα ελέγχου σας.
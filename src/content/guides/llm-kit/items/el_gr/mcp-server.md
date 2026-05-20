Η FastComments φιλοξενεί έναν Model Context Protocol (MCP) server ώστε βοηθήματα κωδικοποίησης AI και agentic πελάτες να μπορούν να καλούν απευθείας το FastComments API. Κάθε εργαλείο που εκθέτει ο MCP server παράγεται αυτόματα από το δημόσιο OpenAPI spec, οπότε ό,τι μπορεί να κάνει το REST API, μπορεί να το κάνει και ένας MCP client.

Το endpoint είναι χωρίς κατάσταση και βασισμένο σε streamable-HTTP. Δεν υπάρχει session που χρειάζεται να διατηρηθεί, δεν υπάρχει βήμα εγγραφής πελάτη και δεν υπάρχει κατάσταση στην πλευρά του server ανά πελάτη.

### Τερματικό

[inline-code-attrs-start title = 'Τερματικό MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Η πιστοποίηση χρησιμοποιεί το ίδιο API key με το REST API. Μπορείτε επίσης να περάσετε το `tenantId` και το κλειδί ως `x-tenant-id` και `x-api-key` HTTP headers αν ο πελάτης σας υποστηρίζει προσαρμοσμένα headers.

### Προ-συμπληρωμένη ρύθμιση

Το dashboard διαθέτει έναν βοηθό ρύθμισης που δημιουργεί το URL και έτοιμα αποσπάσματα διαμόρφωσης για δημοφιλείς MCP clients. Μεταβείτε στο dashboard του λογαριασμού σας και ανοίξτε **Integrate -> MCP Server**, ή επισκεφτείτε το απευθείας:

[inline-code-attrs-start title = 'Σελίδα ρύθμισης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Επιλέξτε ποιο API key θα χρησιμοποιήσετε από το αναπτυσσόμενο μενού, και στη συνέχεια αντιγράψτε οποιοδήποτε από τα παραγόμενα αποσπάσματα.

### Claude Code

Καταχωρήστε τον FastComments server με μία εντολή:

[inline-code-attrs-start title = 'Ρύθμιση Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Αφού καταχωρηθεί, εκτελέστε `/mcp` μέσα σε ένα session του Claude Code για να επιβεβαιώσετε τη σύνδεση και να δείτε τα διαθέσιμα εργαλεία.

### Claude Desktop / Cursor

Προσθέστε αυτό το μπλοκ στη διαμόρφωση των MCP servers του πελάτη σας (`claude_desktop_config.json` για Claude Desktop, `mcp.json` για Cursor):

[inline-code-attrs-start title = 'Διαμόρφωση πελάτη MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Το API key είναι ενσωματωμένο στο URL. Αντιμετωπίστε το URL σαν μυστικό: μην το επικολλάτε σε δημόσιες συνομιλίες, στιγμιότυπα οθόνης ή commits. Εάν ένα κλειδί εκτεθεί, ανανεώστε/αντικαταστήστε το στη σελίδα API Keys στο dashboard σας.
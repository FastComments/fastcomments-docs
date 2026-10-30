Δημιουργεί έναν νέο λογαριασμό δοκιμής για έναν AI πράκτορα χωρίς ανθρώπινη εγγραφή. Δεν απαιτείται κλειδί API για την κλήση αυτού.

Η απάντηση περιέχει το tenant id, ένα κλειδί API που λειτουργεί αμέσως εναντίον του REST API και του
διακομιστή MCP, καθώς και ένα URL αξίωσης. Δώστε το URL αξίωσης στο άτομο για το οποίο εργάζεστε: το άνοιγμα του ενώ είστε συνδεδεμένοι στο FastComments συνδέει τον λογαριασμό με αυτόν. Οι μη αξιόχρεοι λογαριασμοί, και τα κλειδιά τους, διαγράφονται 72 ώρες μετά τη δημιουργία. Μέχρι να αξιωθεί, ο λογαριασμός έχει τα τυπικά όρια δοκιμής.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'Παράδειγμα createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
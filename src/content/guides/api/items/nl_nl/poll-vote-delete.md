[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Trekt een stem in. De optie waarop de stem werd uitgebracht krijgt zijn teller terug, en de kiezer kan opnieuw stemmen.

[inline-code-attrs-start title = 'PollVote Verwijderen cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Verwijderen Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Verwijderen Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Overige opmerkingen

- Het verwijderen van dezelfde stem twee keer geeft `not-found` als reactie de tweede keer, en de tellers blijven ongewijzigd.
- Als de poll sinds het uitbrengen van de stem is vervangen, wordt de stem verwijderd maar verandert er niets aan de tellingen, omdat de vervanging vanaf nul begon.
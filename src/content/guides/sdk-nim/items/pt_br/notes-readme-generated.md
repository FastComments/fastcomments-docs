### IDs de Broadcast

Você verá que deve passar um `broadcastId` em algumas chamadas de API. Quando receber eventos, você receberá esse ID de volta, então saberá ignorar o evento se planeja aplicar alterações de forma otimista no cliente
(o que você provavelmente vai querer fazer, já que oferece a melhor experiência). Passe um UUID aqui. O ID deve ser suficientemente único para não ocorrer duas vezes em uma sessão do navegador.

### SSO (Login Único)

Para exemplos de SSO, veja abaixo.
<?php
/*
	The credentials contain a single message-qop value which is not
	surrounded by double quotes.

		credentials      = "Digest" digest-response
		digest-response  = 1#( username | realm | nonce | digest-uri |
							response | [ algorithm ] | [cnonce] |
							[opaque] | [message-qop] |
							[nonce-count]  | [auth-param] )

		message-qop      = "qop" "=" qop-value

		request-digest  = <"> < KD ( H(A1),     unq(nonce-value)
				":" nc-value
				":" unq(cnonce-value)
				":" unq(qop-value)
				":" H(A2) ) >
			<">

		A1 is:
			A1       = unq(username-value) ":" unq(realm-value) ":" passwd

		This value of A1 is:
			bob:biloxi.com:zanzibar

		A2 is:
			A2       = Method ":" digest-uri-value

		The value of A2 is
			INVITE:sip:bob@biloxi.com

	The following examples, taken from RFC3261 [1], display this subtle
	point.  The WWW-Authenticate header is an example of a challenge,
	whilst the Authorization header is an example of credentials.

		WWW-Authenticate: Digest
				realm="biloxi.com",
				qop="auth,auth-int",
				nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093",
				opaque="5ccc069c403ebaf9f0171e9517f40e41"

		Authorization: Digest username="bob",
				realm="biloxi.com",
				nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093",
				uri="sip:bob@biloxi.com",
				qop=auth,
				nc=00000001,
				cnonce="0a4f113b",
				response="6629fae49393a05397450978507c4ef1",
				opaque="5ccc069c403ebaf9f0171e9517f40e41"


 */
$method = "INVITE";
$username = "bob";
$realm = "biloxi.com";
$passwd = "zanzibar";
$nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093";
$uri="sip:bob@biloxi.com";
$qop="auth";
$nc="00000001";
$cnonce="0a4f113b";
$response="6629fae49393a05397450978507c4ef1";
$opaque="5ccc069c403ebaf9f0171e9517f40e41";


$A1 = "{$username}:{$realm}:{$passwd}";
$A1H = md5($A1);
echo "A1: {$A1} {$A1H}\n";

$A2 = "{$method}:{$uri}";
$A2H = md5($A2);
echo "A2: {$A2} {$A2H}\n";

$challenge  = "{$A1H}:{$nonce}:{$nc}:{$cnonce}:{$qop}:{$A2H}";
$challengeH = md5($challenge);
echo "Challenge: {$challenge} {$challengeH}\n";
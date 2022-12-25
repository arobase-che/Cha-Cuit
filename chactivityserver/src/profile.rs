/**
 * Copyright (c) 2022, Sébastien Blin <sebastien.blin@enconn.fr>
 * All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright
 *  notice, this list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright
 *  notice, this list of conditions and the following disclaimer in the
 *  documentation and/or other materials provided with the distribution.
 * * Neither the name of the University of California, Berkeley nor the
 *  names of its contributors may be used to endorse or promote products
 *  derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE REGENTS AND CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 **/
use crate::config::Config;

use actix_web::{HttpResponse, Responder};
use serde_json::json;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Profile {
  pub config: Config,
}

impl Profile {
    pub fn profile(&self) -> impl Responder {
        let profile_json = json!({
            "@context": [
              "https://www.w3.org/ns/activitystreams",
              "https://w3id.org/security/v1",
              Profile::mastodon_value()
            ],
            "id": format!("https://{}/users/{}", self.config.domain, self.config.user),
            "type": "Person",
            "following": format!("https://{}/users/{}/following", self.config.domain, self.config.user),
            "followers": format!("https://{}/users/{}/followers", self.config.domain, self.config.user),
            "inbox": format!("https://{}/users/{}/inbox", self.config.domain, self.config.user),
            "outbox": format!("https://{}/users/{}/outbox", self.config.domain, self.config.user),
            "featuredTags": format!("https://{}/tags", self.config.domain),
            "preferredUsername": format!("{}", self.config.user),
            "name": format!("{}", self.config.preferred_name),
            "summary": format!("{}", self.config.description),
            "url": format!("https://{}/recettes/", self.config.domain),
            "manuallyApprovesFollowers": self.config.manually_approve_followers,
            "discoverable": self.config.discoverable,
            "published": "2022-11-11T11:11:11Z", // TODO get oldest recipe
            "devices": format!("https://{}/users/{}/collections/devices", self.config.domain, self.config.user), // TODO endpoint
            "publicKey": {
              "id": format!("https://{}/users/{}#main-key", self.config.domain, self.config.user),
              "owner": format!("https://{}/users/{}", self.config.domain, self.config.user),
              "publicKeyPem": std::fs::read_to_string(&*self.config.public_key).unwrap(),
            },
            "tag": [],
            "attachment": [
              {
                "type": "PropertyValue",
                "name": "website",
                "value": format!("<a href=\"https://{}\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\"><span class=\"invisible\">https://</)span><span class=\"\">{}</span><span class=\"invisible\"></span></a>", self.config.domain, self.config.domain)
              }
            ],
            "endpoints": {
              "sharedInbox": format!("https://{}/users/{}/inbox", self.config.domain, self.config.user)
            },
            "icon": {
                "type": "Image",
                "url": format!("https://{}/{}", self.config.domain, self.config.banner)
            },
            "image": {
              "type": "Image",
              "mediaType": "image/jpeg",
              "url": format!("https://{}/{}", self.config.domain, self.config.avatar)
            }
          });
        HttpResponse::Ok().json(profile_json)
    }

    fn mastodon_value() -> Value {
        json!({
            "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
            "toot": "http://joinmastodon.org/ns#",
            "featuredTags": {
              "@id": "toot:featuredTags",
              "@type": "@id"
            },
            "alsoKnownAs": {
              "@id": "as:alsoKnownAs",
              "@type": "@id"
            },
            "movedTo": {
              "@id": "as:movedTo",
              "@type": "@id"
            },
            "schema": "http://schema.org#",
            "PropertyValue": "schema:PropertyValue",
            "value": "schema:value",
            "discoverable": "toot:discoverable",
            "Device": "toot:Device",
            "Ed25519Signature": "toot:Ed25519Signature",
            "Ed25519Key": "toot:Ed25519Key",
            "Curve25519Key": "toot:Curve25519Key",
            "EncryptedMessage": "toot:EncryptedMessage",
            "publicKeyBase64": "toot:publicKeyBase64",
            "deviceId": "toot:deviceId",
            "claim": {
              "@type": "@id",
              "@id": "toot:claim"
            },
            "fingerprintKey": {
              "@type": "@id",
              "@id": "toot:fingerprintKey"
            },
            "identityKey": {
              "@type": "@id",
              "@id": "toot:identityKey"
            },
            "devices": {
              "@type": "@id",
              "@id": "toot:devices"
            },
            "messageFranking": "toot:messageFranking",
            "messageType": "toot:messageType",
            "cipherText": "toot:cipherText",
            "suspended": "toot:suspended",
            "focalPoint": {
              "@container": "@list",
              "@id": "toot:focalPoint"
            }
        })
    }
}
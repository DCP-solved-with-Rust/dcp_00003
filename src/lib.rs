/*
 * Copyright (c) 2018 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

/*
 * The question did not specify a format for the string representation,
 * so we'll pick one ourselves. We choose JSON because it is widespread
 * and it fits the bill in terms of being a string representation capable
 * of de/serializing nested objects. No reason to invent our own syntax I say.
 *
 * We will be using the JSON support of the serialization and deserialization
 * framework Serde. Is this fine or is this cheating? ... On one hand I suppose
 * the purpose of these coding problems is to test our knowledge, and an interviewer
 * might want to see if we understand the fundamentals of the concept of serialization
 * and deserialization. On the other hand, every line of code that we don't have to
 * write is one less line of code to worry about maintaining, and also why waste time
 * rewriting something that already exists if the existing thing fits the bill?
 *
 * Lines of code add up, and the pain of maintaining a codebase is real. So I am
 * going to go with "let's use an appropriate framework or library" for $500 :)
 */

#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct Node
{
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn serialize (root: Node) -> String
{
    return serde_json::to_string(&root).unwrap();
}

fn deserialize (s: String) -> Node
{
    return serde_json::from_str(s.as_str()).unwrap();
}

#[test]
fn test ()
{
    let node = Node
    {
        val: "root".to_string(),
        left: Some(Box::new(Node
        {
            val: "left".to_string(),
            left: Some(Box::new(Node
            {
                val: "left.left".to_string(),
                left: None,
                right: None
            })),
            right: None,
        })),
        right: None
    };

    assert_eq!(deserialize(serialize(node)).left.unwrap().left.unwrap().val, "left.left");
}

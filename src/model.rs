use chrono::{NaiveDateTime, Utc};

use crate::util;

#[derive(Debug)]
/// Combined model for a syndication feed (i.e. RSS1, RSS 2, Atom)
///
/// The model is based on the Atom standard as a start with RSS1+2 mapped on to it
/// Atom:
///     Feed -> Feed, Entry -> Entry
/// RSS 1 + 2:
///     Channel -> Feed, Item -> Entry
///
/// Atom spec: http://www.atomenabled.org/developers/syndication/
/// RSS 2 spec: https://validator.w3.org/feed/docs/rss2.html
/// RSS 1 spec: https://validator.w3.org/feed/docs/rss1.html
/// 
/// Certain elements are not mapped given their limited utility:
///   * RSS 2:
///     * channel - docs (pointer to the spec), cloud (for callbacks), textInput (text box e.g. for search)
///     * item - comments (link to comments on the article), source (pointer to the channel, but our data model links items to a channel)
///   * RSS 1:
///     * channel - rdf:about attribute (pointer to feed), textinput (text box e.g. for search)
pub struct Feed {
    /// Atom (required): Identifies the feed using a universally unique and permanent URI.
    pub id: String,
    /// Atom (required): Contains a human readable title for the feed. Often the same as the title of the associated website. This value should not be blank.
    /// RSS 1 + 2 (required) "title": The name of the channel. It's how people refer to your service.
    pub title: String,
    /// Atom (required): Indicates the last time the feed was modified in a significant way.
    /// RSS 2 (optional) "lastBuildDate": The last time the content of the channel changed.
    pub updated: NaiveDateTime,

    /// Atom (recommended): Collection of authors defined at the feed level.
    /// RSS 2 (optional) "managingEditor": Email address for person responsible for editorial content.
    pub authors: Vec<Person>,
    /// RSS 1 + 2 (required): Phrase or sentence describing the channel.
    pub description: Option<String>,
    /// Atom (recommended): Identifies a related Web page.
    /// RSS 1 + 2 (required): The URL to the HTML website corresponding to the channel.
    pub link: Option<Link>,

    /// Atom (optional): Specifies a category that the feed belongs to. A feed may have multiple category elements.
    /// RSS 2 (optional) "category": Specify one or more categories that the channel belongs to.
    pub categories: Vec<Category>,
    /// Atom (optional): Names one contributor to the feed. A feed may have multiple contributor elements.
    /// RSS 2 (optional) "webMaster": Email address for person responsible for technical issues relating to channel.
    pub contributors: Vec<Person>,
    /// Atom (optional): Identifies the software used to generate the feed, for debugging and other purposes.
    /// RSS 2 (optional): A string indicating the program used to generate the channel.
    pub generator: Option<Generator>,
    /// Atom (optional): Identifies a small image which provides iconic visual identification for the feed.
    pub icon: Option<String>,
    /// RSS 2 (optional): The language the channel is written in.
    pub language: Option<String>,
    /// Atom (optional): Identifies a larger image which provides visual identification for the feed.
    /// RSS 1 + 2 (optional) "image": Specifies a GIF, JPEG or PNG image that can be displayed with the channel.
    pub logo: Option<Image>,
    /// RSS 2 (optional): The publication date for the content in the channel.
    pub pub_date: Option<NaiveDateTime>,
    /// Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
    /// RSS 2 (optional) "copyright": Copyright notice for content in the channel.
    pub rights: Option<String>,
    /// Atom (optional): Contains a human-readable description or subtitle for the feed.
    pub subtitle: Option<String>,
    /// RSS 2 (optional): It's a number of minutes that indicates how long a channel can be cached before refreshing from the source.
    pub ttl: Option<u32>,

    /// Atom (optional): Individual entries within the feed (e.g. a blog post)
    /// RSS 1+2 (optional): Individual items within the channel.
    pub entries: Vec<Entry>,
}

impl Feed {
    fn new() -> Self {
        let id = util::uuid_gen();
        let title = format!("feed: {}", id);

        Feed {
            id,
            title,
            updated: Utc::now().naive_utc(),
            authors: Vec::new(),
            description: None,
            link: None,
            categories: Vec::new(),
            contributors: Vec::new(),
            generator: None,
            icon: None,
            language: None,
            logo: None,
            pub_date: None,
            rights: None,
            subtitle: None,
            ttl: None,
            entries: Vec::new(),
        }
    }
}

/// An item within a feed
#[derive(Debug)]
pub struct Entry {
    /// Atom (required): Identifies the entry using a universally unique and permanent URI.
    /// RSS 2 (optional) "guid": A string that uniquely identifies the item.
    pub id: String,
    /// Atom, RSS 1(required): Contains a human readable title for the entry.
    /// RSS 2 (optional): The title of the item.
    pub title: String,
    /// Atom (required): Indicates the last time the entry was modified in a significant way.
    pub updated: NaiveDateTime,

    /// Atom (recommended): Collection of authors defined at the entry level.
    /// RSS 2 (optional): Email address of the author of the item.
    pub authors: Vec<Person>,
    /// Atom (recommended): Contains or links to the complete content of the entry.
    /// RSS 2 (optional) "enclosure": Describes a media object that is attached to the item.
    pub content: Option<Content>,
    /// Atom (recommended): Identifies a related Web page.
    /// RSS 2 (optional): The URL of the item.
    /// RSS 1 (required): The item's URL.
    pub link: Option<Link>,
    /// Atom (recommended): Conveys a short summary, abstract, or excerpt of the entry.
    /// RSS 1+2 (optional): The item synopsis.
    pub summary: Option<String>,

    /// Atom (optional): Specifies a category that the entry belongs to. A feed may have multiple category elements.
    /// RSS 2 (optional): Includes the item in one or more categories.
    pub categories: Vec<Category>,
    /// Atom (optional): Names one contributor to the entry. A feed may have multiple contributor elements.
    pub contributors: Vec<Person>,
    /// Atom (optional): Contains the time of the initial creation or first availability of the entry.
    /// RSS 2 (optional) "pubDate": Indicates when the item was published.
    pub published: Option<NaiveDateTime>,
    /// Atom (optional): If an entry is copied from one feed into another feed, then this contains the source feed metadata.
    pub source: Option<String>,
    /// Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
    pub rights: Option<String>,
}

impl Entry {
    fn new() -> Self {
        let id = util::uuid_gen();
        let title = format!("entry: {}", id);

        Entry {
            id,
            title,
            updated: Utc::now().naive_utc(),
            authors: Vec::new(),
            content: None,
            link: None,
            summary: None,
            categories: Vec::new(),
            contributors: Vec::new(),
            published: None,
            source: None,
            rights: None,
        }
    }
}

/// Represents the category of a feed or entry
/// Atom spec: http://www.atomenabled.org/developers/syndication/#category
/// RSS 2 spec: https://validator.w3.org/feed/docs/rss2.html#ltcategorygtSubelementOfLtitemgt
#[derive(Debug)]
pub struct Category {
    /// Atom (required): Identifies the category.
    pub term: String,
    /// Atom (optional): Identifies the categorization scheme via a URI.
    pub scheme: Option<String>,
    /// Atom (optional): Provides a human-readable label for display.
    pub label: Option<String>,
}

impl Category {
    pub fn new(term: String) -> Category {
        Category { term, scheme: None, label: None }
    }
}

/// The content, or link to the content, for a given entry.
/// Atom spec: http://www.atomenabled.org/developers/syndication/#contentElement
/// RSS 2 spec: https://validator.w3.org/feed/docs/rss2.html#ltenclosuregtSubelementOfLtitemgt
#[derive(Debug)]
pub struct Content {
    /// Atom: The type attribute is either text, html, xhtml, in which case the content element is defined identically to other text constructs.
   /// TODO enum
    pub content_type: Option<String>,
    /// Atom: If the src attribute is present, it represents the URI of where the content can be found. The type attribute, if present, is the media type of the content.
    pub src: Option<String>,
    /// Atom:
    ///     If the type attribute ends in +xml or /xml, then an xml document of this type is contained inline.
    ///     If the type attribute starts with text, then an escaped document of this type is contained inline.
    ///     Otherwise a base64 encoded document of the indicated media type is contained inline.
    // TODO enum
    pub inline: Option<String>,
}

impl Content {
    pub fn new() -> Content {
        Content { content_type: None, src: None, inline: None }
    }
}

/// Information on the tools used to generate the feed
/// Atom: Identifies the software used to generate the feed, for debugging and other purposes.
#[derive(Debug)]
pub struct Generator {
    /// Atom: Link to the tool
    pub uri: Option<String>,
    /// Atom: Tool version
    pub version: Option<String>,
    /// Atom: Additional data
    pub inline: Option<String>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator { uri: None, version: None, inline: None }
    }
}

/// Represents a a link to an image.
/// RSS 2 spec: https://validator.w3.org/feed/docs/rss2.html#ltimagegtSubelementOfLtchannelgt
/// RSS 1 spec: https://validator.w3.org/feed/docs/rss1.html#s5.4
#[derive(Debug)]
pub struct Image {
    /// RSS 1 + 2: the URL of a GIF, JPEG or PNG image that represents the channel.
    pub url: String,
    /// RSS 1 + 2: describes the image, it's used in the ALT attribute of the HTML <img> tag when the channel is rendered in HTML.
    pub title: String,
    /// RSS 1 + 2: the URL of the site, when the channel is rendered, the image is a link to the site.
    pub link: Link,

    /// RSS 2 (optional): width of the image, defaults to 88, max 144
    pub width: u32,
    /// RSS 2 (optional): height of the image, defaults to 31, max 400
    pub height: u32,
    /// RSS 2 (optional): contains text that is included in the TITLE attribute of the link formed around the image in the HTML rendering.
    pub description: Option<String>,
}

impl Image {
    pub fn new(url: String, title: String, link: Link) -> Image {
        Image { url, title, link, width: 88, height: 31, description: None}
    }
}

/// Represents a link to an associated resource for the feed or entry.
/// Atom spec: http://www.atomenabled.org/developers/syndication/#link
#[derive(Debug)]
pub struct Link {
    /// The URI of the referenced resource (typically a Web page).
    pub href: String,
    /// A single link relationship type.
    pub rel: Option<String>,
    /// Indicates the media type of the resource.
    pub media_type: Option<String>,
    /// Indicates the language of the referenced resource.
    pub hreflang: Option<String>,
    /// Human readable information about the link, typically for display purposes.
    pub title: Option<String>,
    /// The length of the resource, in bytes.
    pub length: Option<u64>,
}

impl Link {
    pub fn new(href: String) -> Link {
        Link {
            href,
            rel: None,
            media_type: None,
            hreflang: None,
            title: None,
            length: None,
        }
    }
}

/// Represents an author, contributor etc.
/// Atom spec: http://www.atomenabled.org/developers/syndication/#person
#[derive(Debug)]
pub struct Person {
    /// Atom: human-readable name for the person.
    pub name: String,
    /// Atom: home page for the person.
    pub uri: Option<String>,
    /// Atom: An email address for the person.
    pub email: Option<String>,
}

impl Person {
    pub fn new(name: String) -> Person {
        Person { name, uri: None, email: None }
    }
}
This project is a stash for thoughts based on the Zettelkasten method. 

The project is composed of a Backend service reachable through a HTTP API.

# Product description 

## Story Mapping

### Business language

`User` → `Scribe`:
A Scribe interacts with Notes, Thoughts and Questions. He creates Projects.

`Piece of Information (PoI)`:
a Piece of Information is either a Note, a Thought or a Question. All PoI can contain a short piece of textual information that can be accompanied by one or several medias (images, sounds etc.) and can contain links.

`Note`:
A Note is a PoI that is imported from an external source. This may be notes taken from a meeting or a book etc.

`Thought`:
A Thought is a PoI that can be seen as the digest of a Note or a previous Thought or answer to a Question.

`Question`:
A Question is a way to open a new search field from previous information (Thoughts or Notes). 

`Project`:
All pieces of information belong to a Project. Notes are often created in very pragmatic context (reading of a book, work project etc.) whereas some projects may become more theoretical (ie `life`) with Thoughts and Questions far more general.

### Processes

A Scribe posts Notes in a Project. He can view all notes in a given Project. He can then create Thoughts and Questions and discard Notes. A Thought can be refuted by another Thought. 

### Taxonomy

There are several ways Pieces of Information (PoI) can be searched:
 * full text search
 * nested categories (not Notes)
 * tags (plain tags)
 * projects
 * references (books mostly)
 * links (find all PoI that link to a given PoI)

Each Thought or Question can relate to a parent Thought or Question and this creates trees of PoI.

There can be linked Thoughts or Questions (not Notes) on a PoI.

## Data Structures

### Note

 * imported_at Timestamp
 * scribe_id   ScribeIdentifier
 * project_id  ProjectIdentifier
 * content     text

### Thought

 * thought_id           ThoughtIdentifier
 * parent_thought_id    Option(ThoughtIdentifier)
 * refuted_by           Option(ThoughtIdentifier)
 * created_at           Timestamp
 * scribe_id            ScribeIdentifier
 * project_id           ProjectIdentifier
 * variation            ThoughtVariation (Thought or Question)
 * content              text
 * tags                 text[]
 * categories           Category[]
 * media                Media[]
 * references           Reference[]
 * links                ThoughtIdentifier[]

### Project

 * project_id   ProjectIdentifier
 * universe_id  UniverseIdentifier
 * created_at   Timestamp
 * name         text
 * locked       boolean

### Universe

 * universe_id      UniverseIdentifier
 * organization_id  OrganizationIdentifier
 * is_private       boolean

### Organization

 * organization_id  OrganizationIdentifier
 * name             text

### Scribe

 * scribe_id        ScribeIdentifier
 * organization_id  OrganizationIdentifier
 * display_name     text
 * email_address    text

## Value measurement 

Value can be measured by the work done on Notes. 

 * average notes input per day
 * average note lifetime

In the long term, the efficiency of the Search feature is the key of success:

 * average number of results per query
 * average number of queries per work session

## Commands

### Note creation

Command: CreateNoteCommand

Attributes:
- imported_at: timestamp
- scribe_id: uuid
- project_id: uuid
- content: text

Validation:
- Ensure `imported_at` is a valid timestamp.
- Ensure `scribe_id` is a valid integer and references an existing scribe.
- Ensure `project_id` is a valid integer and references an existing project.
- Ensure `content` is not empty.

Execution: Insert a new note in the notes book.

## Queries

### Notes

Query: FetchNotesByProjectQuery

Parameters:
- project_id: integer

Execution:
- Select all records from the `note` table where `project_id` matches the provided parameter.

Query: FetchNoteByIdQuery

Parameters:
- note_id: integer

Execution:
- Select the record from the `note` table where `id` matches the provided parameter.

# Software architecture

3 API systems:
* OpenMind: that manages thoughts and scribes.
* Scrooge: that manages invoicing.
* SynApps: Event dispatcher for API systems.

## Services and Actors

### OpenMind application

services:
 * thoughts 
 * scribes
 * thought_search 
 * organization

actors:
 * api 
 * event_logger

### Scrooge application

services:
 * organization
 * invoice
 
 actors:
 * api
 * event_logger
 * time_keeper
 * accountant

## Database

Both applications use their own PostgreSQL database. The database uses the Trigram extension alongside the full text search with dedicated tables to denormalize data for fast search. The extension LTree is used to store categories.

# Hosting 

## Network

### Security

The API security will be handled by a [Biscuit](https://www.biscuitsec.org/). Each HTTP request will embed a token signed by the client which will be used to authenticate the scribe. 

## Logging
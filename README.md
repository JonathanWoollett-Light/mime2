# Mime2

[![Crates.io](https://img.shields.io/crates/v/mime2)](https://crates.io/crates/mime2)
[![docs](https://img.shields.io/crates/v/mime2?color=yellow&label=docs)](https://docs.rs/mime2)

MIME types that are more up to date than [mime](https://github.com/hyperium/mime).

### How to update

Update all the CSVs from https://www.iana.org/assignments/media-types/media-types.xhtml.

- `application.csv`: https://www.iana.org/assignments/media-types/application.csv
- `font.csv`: https://www.iana.org/assignments/media-types/font.csv
- `haptics.csv`: https://www.iana.org/assignments/media-types/haptics.csv
- `image.csv`: https://www.iana.org/assignments/media-types/image.csv
- `message.csv`: https://www.iana.org/assignments/media-types/message.csv
- `model.csv`: https://www.iana.org/assignments/media-types/model.csv
- `multipart.csv`: https://www.iana.org/assignments/media-types/multipart.csv
- `text.csv`: https://www.iana.org/assignments/media-types/text.csv
- `video.csv`: https://www.iana.org/assignments/media-types/video.csv

### Why is manually updating required?

If you attempt to do a simple get reqwest on the url you receive:

> Status 403 Forbidden: User-Agent required. Contact iana@iana.org with questions.

Attempting circumvent this are presently out of scope.
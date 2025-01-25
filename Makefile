.PHONY: build build-release test test-coverage validate format docker-build

build:
	@cargo build

build-release:
	@cargo build --release

test:
	@cargo test

test-coverage:
	@cargo tarpaulin

validate:
	@xmllint --noout --schema xml/xsd/mobilesync/AutodiscoverRequest.xsd xml/mobilesync/AutodiscoverRequest.xml
	@xmllint --noout --schema xml/xsd/mobilesync/AutodiscoverResponse.xsd templates/xml/autodiscover-mobilesync.xml.tera

	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverRequest.xsd xml/autodiscover/AutodiscoverRequest.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponse.xsd xml/autodiscover/AutodiscoverExchangeResponse.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponse.xsd xml/autodiscover/AutodiscoverResponse.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponse.xsd templates/xml/autodiscover.xml.tera
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverExchangeResponseRedirect.xsd xml/autodiscover/AutodiscoverResponseRedirect.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponseError.xsd xml/autodiscover/AutodiscoverResponseError.xml

format:
	@cargo fmt -- --emit files

export function isValidStellarAddress(address: string): boolean {
  return /^G[A-Z2-7]{55}$/.test(address);
}

export function downloadJSON(data: object, filename: string) {
  const json = JSON.stringify(data, null, 2);
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

export function downloadXML(data: object, filename: string) {
  const xml = objectToXml(data);
  const blob = new Blob([xml], { type: "application/xml" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

function objectToXml(obj: any, rootName = "root"): string {
  let xml = '<?xml version="1.0" encoding="UTF-8"?>\n';
  xml += `<${rootName}>\n`;
  xml += objectToXmlContent(obj, 1);
  xml += `</${rootName}>`;
  return xml;
}

function objectToXmlContent(obj: any, indent: number): string {
  const spaces = "  ".repeat(indent);
  let xml = "";

  for (const [key, value] of Object.entries(obj)) {
    const tagName = key.replace(/[^a-zA-Z0-9_]/g, "_");
    if (value === null || value === undefined) {
      xml += `${spaces}<${tagName} />\n`;
    } else if (typeof value === "object" && !Array.isArray(value)) {
      xml += `${spaces}<${tagName}>\n`;
      xml += objectToXmlContent(value, indent + 1);
      xml += `${spaces}</${tagName}>\n`;
    } else if (Array.isArray(value)) {
      value.forEach((item) => {
        xml += `${spaces}<item>\n`;
        if (typeof item === "object") {
          xml += objectToXmlContent(item, indent + 1);
        } else {
          xml += `${"  ".repeat(indent + 1)}${item}\n`;
        }
        xml += `${spaces}</item>\n`;
      });
    } else {
      xml += `${spaces}<${tagName}>${String(value)}</${tagName}>\n`;
    }
  }

  return xml;
}

export function copyToClipboard(text: string): Promise<void> {
  return navigator.clipboard.writeText(text);
}

"use client";

import { ContentManifest } from "@stellar-veriphy/shared/types";
import { jsonToXml } from "@/utils/manifestConverter";

interface ManifestPreviewProps {
  manifest: ContentManifest;
  format: "json" | "xml";
}

export function ManifestPreview({ manifest, format }: ManifestPreviewProps) {
  const content =
    format === "json"
      ? JSON.stringify(manifest, null, 2)
      : jsonToXml(manifest, "manifest");

  return (
    <pre className="bg-gray-900 text-gray-100 p-4 rounded overflow-auto max-h-96 text-sm font-mono">
      <code>{content}</code>
    </pre>
  );
}

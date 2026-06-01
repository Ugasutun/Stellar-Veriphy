export async function hashFile(
  file: File,
  onProgress?: (progress: number) => void
): Promise<string> {
  const chunkSize = 64 * 1024; // 64KB chunks
  const chunks = Math.ceil(file.size / chunkSize);
  let hash = await crypto.subtle.digest("SHA-256", new Uint8Array(0));

  for (let i = 0; i < chunks; i++) {
    const start = i * chunkSize;
    const end = Math.min(start + chunkSize, file.size);
    const chunk = file.slice(start, end);
    const buffer = await chunk.arrayBuffer();
    hash = await crypto.subtle.digest("SHA-256", buffer);
    if (onProgress) {
      onProgress(((i + 1) / chunks) * 100);
    }
  }

  return Array.from(new Uint8Array(hash))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

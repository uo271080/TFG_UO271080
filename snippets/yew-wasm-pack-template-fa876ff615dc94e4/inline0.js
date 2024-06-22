
export function exportCsv(csvContent, fileName) {
    // Crear un Blob con el contenido del CSV
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });

    // Crear un enlace para la descarga
    const link = document.createElement('a');

    // Usar URL.createObjectURL para obtener una URL para el blob
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', fileName);

    // Asegurarse que el link sea no visible y añadirlo al DOM
    link.style.visibility = 'hidden';
    document.body.appendChild(link);

    // Hacer clic en el enlace para descargar el archivo
    link.click();

    // Limpiar y remover el enlace del DOM
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
}

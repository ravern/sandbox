export default function getObjects<T extends { id: string }>(
  objects: { [id: string]: T },
  predicate: (object: T) => boolean,
): { [id: string]: T } {
  const filteredObjects = {};
  Object.values(objects).forEach(object => {
    if (predicate(object)) {
      filteredObjects[object.id] = object;
    }
  });
  return filteredObjects;
}

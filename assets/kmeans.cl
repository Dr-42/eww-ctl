__kernel void kmeans_cluster(
    __global const uchar4* image,
    __global const float3* centroids,
    __global uint* cluster_assignments,
    const uint num_centroids,
    const uint num_pixels
) {
    int i = get_global_id(0);
    if (i >= num_pixels) return;

    uchar4 pixel = image[i];
    float3 color = (float3)(pixel.x, pixel.y, pixel.z);

    float min_distance = FLT_MAX;
    uint best_centroid = 0;

    for (uint c = 0; c < num_centroids; ++c) {
        float3 diff = color - centroids[c];
        float distance = dot(diff, diff);
        if (distance < min_distance) {
            min_distance = distance;
            best_centroid = c;
        }
    }
    cluster_assignments[i] = best_centroid;
}

import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useProfileStore } from '@/stores/profile-store';
import { usePostStore } from '@/stores/post-store';

// Document event types
interface UserProfileEvent {
  type: 'local_insert' | 'remote_insert';
  key: number[];
  author: string;
  from?: string;
}

interface UserContentReadyEvent {
  hash: string;
}

interface UserNeighborEvent {
  type: 'neighbor_up' | 'neighbor_down';
  node_id: string;
}

interface UserSyncEvent {
  origin: string;
}

interface PostContentEvent {
  type: 'local_insert' | 'remote_insert';
  key: number[];
  author: string;
  from?: string;
}

interface PostContentReadyEvent {
  hash: string;
}

interface PostNeighborEvent {
  type: 'neighbor_up' | 'neighbor_down';
  node_id: string;
}

interface PostSyncEvent {
  origin: string;
}

// Document Events Hook
export const useDocumentEvents = () => {
  const { refreshUser, setNetworkStatus } = useProfileStore();
  const { refreshPosts, setNetworkStatus: setPostNetworkStatus } = usePostStore();

  useEffect(() => {
    let unlistenUserProfile: (() => void) | undefined;
    let unlistenUserContent: (() => void) | undefined;
    let unlistenUserNeighbor: (() => void) | undefined;
    let unlistenUserSync: (() => void) | undefined;
    let unlistenPostContent: (() => void) | undefined;
    let unlistenPostContentReady: (() => void) | undefined;
    let unlistenPostNeighbor: (() => void) | undefined;
    let unlistenPostSync: (() => void) | undefined;

    const setupListeners = async () => {
      try {
        // User document events
        unlistenUserProfile = await listen<UserProfileEvent>('user-profile-updated', (event) => {
          console.log('User profile updated:', event.payload);
          // Refresh user data when profile changes are detected
          // For now, we'll need to get the current user ID from auth context
          // TODO: Pass user ID from the event payload or get from auth store
          if (event.payload.author) {
            refreshUser(event.payload.author);
          }
        });

        unlistenUserContent = await listen<UserContentReadyEvent>('user-content-ready', (event) => {
          console.log('User content ready:', event.payload);
          // Content is ready - no specific action needed unless we have user context
        });

        unlistenUserNeighbor = await listen<UserNeighborEvent>('user-neighbor-status', (event) => {
          console.log('User neighbor status:', event.payload);
          // Update network status based on neighbor events
          if (event.payload.type === 'neighbor_up') {
            setNetworkStatus('connected');
          } else if (event.payload.type === 'neighbor_down') {
            setNetworkStatus('disconnected');
          }
        });

        unlistenUserSync = await listen<UserSyncEvent>('user-sync-finished', (event) => {
          console.log('User sync finished:', event.payload);
          // Sync finished - content may have been updated
          setNetworkStatus('connected');
        });

        // Post document events
        unlistenPostContent = await listen<PostContentEvent>('post-content-updated', (event) => {
          console.log('Post content updated:', event.payload);
          // Refresh posts when new content is detected
          refreshPosts();
        });

        unlistenPostContentReady = await listen<PostContentReadyEvent>('post-content-ready', (event) => {
          console.log('Post content ready:', event.payload);
          // Refresh posts when new content is ready
          refreshPosts();
        });

        unlistenPostNeighbor = await listen<PostNeighborEvent>('post-neighbor-status', (event) => {
          console.log('Post neighbor status:', event.payload);
          // Update post network status
          if (event.payload.type === 'neighbor_up') {
            setPostNetworkStatus('connected');
          } else if (event.payload.type === 'neighbor_down') {
            setPostNetworkStatus('disconnected');
          }
        });

        unlistenPostSync = await listen<PostSyncEvent>('post-sync-finished', (event) => {
          console.log('Post sync finished:', event.payload);
          // Refresh posts after sync completion
          refreshPosts();
        });

        console.log('Document event listeners set up successfully');
      } catch (error) {
        console.error('Failed to set up document event listeners:', error);
      }
    };

    setupListeners();

    // Cleanup function
    return () => {
      if (unlistenUserProfile) unlistenUserProfile();
      if (unlistenUserContent) unlistenUserContent();
      if (unlistenUserNeighbor) unlistenUserNeighbor();
      if (unlistenUserSync) unlistenUserSync();
      if (unlistenPostContent) unlistenPostContent();
      if (unlistenPostContentReady) unlistenPostContentReady();
      if (unlistenPostNeighbor) unlistenPostNeighbor();
      if (unlistenPostSync) unlistenPostSync();
    };
  }, [refreshUser, refreshPosts, setNetworkStatus, setPostNetworkStatus]);
};

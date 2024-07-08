/* tslint:disable */
/* eslint-disable */
/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit the class manually.
 *
 * Jellyfin API
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


import { BaseItem } from './base-item';
import { BaseItemDto } from './base-item-dto';
import { ClientCapabilities } from './client-capabilities';
import { GeneralCommandType } from './general-command-type';
import { PlayerStateInfo } from './player-state-info';
import { QueueItem } from './queue-item';
import { SessionUserInfo } from './session-user-info';
import { TranscodingInfo } from './transcoding-info';

/**
 * Class SessionInfo.
 * @export
 * @interface SessionInfo
 */
export interface SessionInfo {
    /**
     * 
     * @type {PlayerStateInfo}
     * @memberof SessionInfo
     */
    'PlayState'?: PlayerStateInfo;
    /**
     * 
     * @type {Array<SessionUserInfo>}
     * @memberof SessionInfo
     */
    'AdditionalUsers'?: Array<SessionUserInfo> | null;
    /**
     * 
     * @type {ClientCapabilities}
     * @memberof SessionInfo
     */
    'Capabilities'?: ClientCapabilities;
    /**
     * Gets or sets the remote end point.
     * @type {string}
     * @memberof SessionInfo
     */
    'RemoteEndPoint'?: string | null;
    /**
     * Gets the playable media types.
     * @type {Array<string>}
     * @memberof SessionInfo
     */
    'PlayableMediaTypes'?: Array<string> | null;
    /**
     * Gets or sets the id.
     * @type {string}
     * @memberof SessionInfo
     */
    'Id'?: string | null;
    /**
     * Gets or sets the user id.
     * @type {string}
     * @memberof SessionInfo
     */
    'UserId'?: string;
    /**
     * Gets or sets the username.
     * @type {string}
     * @memberof SessionInfo
     */
    'UserName'?: string | null;
    /**
     * Gets or sets the type of the client.
     * @type {string}
     * @memberof SessionInfo
     */
    'Client'?: string | null;
    /**
     * Gets or sets the last activity date.
     * @type {string}
     * @memberof SessionInfo
     */
    'LastActivityDate'?: string;
    /**
     * Gets or sets the last playback check in.
     * @type {string}
     * @memberof SessionInfo
     */
    'LastPlaybackCheckIn'?: string;
    /**
     * Gets or sets the name of the device.
     * @type {string}
     * @memberof SessionInfo
     */
    'DeviceName'?: string | null;
    /**
     * Gets or sets the type of the device.
     * @type {string}
     * @memberof SessionInfo
     */
    'DeviceType'?: string | null;
    /**
     * 
     * @type {BaseItemDto}
     * @memberof SessionInfo
     */
    'NowPlayingItem'?: BaseItemDto;
    /**
     * 
     * @type {BaseItem}
     * @memberof SessionInfo
     */
    'FullNowPlayingItem'?: BaseItem;
    /**
     * 
     * @type {BaseItemDto}
     * @memberof SessionInfo
     */
    'NowViewingItem'?: BaseItemDto;
    /**
     * Gets or sets the device id.
     * @type {string}
     * @memberof SessionInfo
     */
    'DeviceId'?: string | null;
    /**
     * Gets or sets the application version.
     * @type {string}
     * @memberof SessionInfo
     */
    'ApplicationVersion'?: string | null;
    /**
     * 
     * @type {TranscodingInfo}
     * @memberof SessionInfo
     */
    'TranscodingInfo'?: TranscodingInfo;
    /**
     * Gets a value indicating whether this instance is active.
     * @type {boolean}
     * @memberof SessionInfo
     */
    'IsActive'?: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof SessionInfo
     */
    'SupportsMediaControl'?: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof SessionInfo
     */
    'SupportsRemoteControl'?: boolean;
    /**
     * 
     * @type {Array<QueueItem>}
     * @memberof SessionInfo
     */
    'NowPlayingQueue'?: Array<QueueItem> | null;
    /**
     * 
     * @type {Array<BaseItemDto>}
     * @memberof SessionInfo
     */
    'NowPlayingQueueFullItems'?: Array<BaseItemDto> | null;
    /**
     * 
     * @type {boolean}
     * @memberof SessionInfo
     */
    'HasCustomDeviceName'?: boolean;
    /**
     * 
     * @type {string}
     * @memberof SessionInfo
     */
    'PlaylistItemId'?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SessionInfo
     */
    'ServerId'?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SessionInfo
     */
    'UserPrimaryImageTag'?: string | null;
    /**
     * Gets the supported commands.
     * @type {Array<GeneralCommandType>}
     * @memberof SessionInfo
     */
    'SupportedCommands'?: Array<GeneralCommandType> | null;
}

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


import { BaseItemDto } from './base-item-dto';
import { KeepUntil } from './keep-until';
import { RecordingStatus } from './recording-status';

/**
 * 
 * @export
 * @interface TimerInfoDto
 */
export interface TimerInfoDto {
    /**
     * Gets or sets the Id of the recording.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'Id'?: string | null;
    /**
     * 
     * @type {string}
     * @memberof TimerInfoDto
     */
    'Type'?: string | null;
    /**
     * Gets or sets the server identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ServerId'?: string | null;
    /**
     * Gets or sets the external identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ExternalId'?: string | null;
    /**
     * Gets or sets the channel id of the recording.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ChannelId'?: string;
    /**
     * Gets or sets the external channel identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ExternalChannelId'?: string | null;
    /**
     * Gets or sets the channel name of the recording.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ChannelName'?: string | null;
    /**
     * 
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ChannelPrimaryImageTag'?: string | null;
    /**
     * Gets or sets the program identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ProgramId'?: string | null;
    /**
     * Gets or sets the external program identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ExternalProgramId'?: string | null;
    /**
     * Gets or sets the name of the recording.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'Name'?: string | null;
    /**
     * Gets or sets the description of the recording.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'Overview'?: string | null;
    /**
     * Gets or sets the start date of the recording, in UTC.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'StartDate'?: string;
    /**
     * Gets or sets the end date of the recording, in UTC.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'EndDate'?: string;
    /**
     * Gets or sets the name of the service.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ServiceName'?: string | null;
    /**
     * Gets or sets the priority.
     * @type {number}
     * @memberof TimerInfoDto
     */
    'Priority'?: number;
    /**
     * Gets or sets the pre padding seconds.
     * @type {number}
     * @memberof TimerInfoDto
     */
    'PrePaddingSeconds'?: number;
    /**
     * Gets or sets the post padding seconds.
     * @type {number}
     * @memberof TimerInfoDto
     */
    'PostPaddingSeconds'?: number;
    /**
     * Gets or sets a value indicating whether this instance is pre padding required.
     * @type {boolean}
     * @memberof TimerInfoDto
     */
    'IsPrePaddingRequired'?: boolean;
    /**
     * Gets or sets the Id of the Parent that has a backdrop if the item does not have one.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ParentBackdropItemId'?: string | null;
    /**
     * Gets or sets the parent backdrop image tags.
     * @type {Array<string>}
     * @memberof TimerInfoDto
     */
    'ParentBackdropImageTags'?: Array<string> | null;
    /**
     * Gets or sets a value indicating whether this instance is post padding required.
     * @type {boolean}
     * @memberof TimerInfoDto
     */
    'IsPostPaddingRequired'?: boolean;
    /**
     * 
     * @type {KeepUntil}
     * @memberof TimerInfoDto
     */
    'KeepUntil'?: KeepUntil;
    /**
     * 
     * @type {RecordingStatus}
     * @memberof TimerInfoDto
     */
    'Status'?: RecordingStatus;
    /**
     * Gets or sets the series timer identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'SeriesTimerId'?: string | null;
    /**
     * Gets or sets the external series timer identifier.
     * @type {string}
     * @memberof TimerInfoDto
     */
    'ExternalSeriesTimerId'?: string | null;
    /**
     * Gets or sets the run time ticks.
     * @type {number}
     * @memberof TimerInfoDto
     */
    'RunTimeTicks'?: number | null;
    /**
     * 
     * @type {BaseItemDto}
     * @memberof TimerInfoDto
     */
    'ProgramInfo'?: BaseItemDto;
}

